use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::envelope;
use crate::error::KiCadError;
use crate::model::common::{DocumentSpecifier, DocumentType, ProjectInfo, VersionInfo};
use crate::proto::kiapi::common::commands as common_commands;
use crate::proto::kiapi::common::types as common_types;
use crate::transport::Transport;

const KICAD_API_SOCKET_ENV: &str = "KICAD_API_SOCKET";
const KICAD_API_TOKEN_ENV: &str = "KICAD_API_TOKEN";

const CMD_PING: &str = "kiapi.common.commands.Ping";
const CMD_GET_VERSION: &str = "kiapi.common.commands.GetVersion";
const CMD_GET_OPEN_DOCUMENTS: &str = "kiapi.common.commands.GetOpenDocuments";

const RES_GET_VERSION: &str = "kiapi.common.commands.GetVersionResponse";
const RES_GET_OPEN_DOCUMENTS: &str = "kiapi.common.commands.GetOpenDocumentsResponse";

#[derive(Clone, Debug)]
pub struct KiCadClient {
    inner: Arc<ClientInner>,
}

#[derive(Debug)]
struct ClientInner {
    transport: Transport,
    token: Mutex<String>,
    client_name: String,
    timeout: Duration,
    socket_uri: String,
}

#[derive(Clone, Debug)]
struct ClientConfig {
    timeout: Duration,
    socket_uri: Option<String>,
    token: Option<String>,
    client_name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig {
                timeout: Duration::from_millis(3_000),
                socket_uri: None,
                token: None,
                client_name: None,
            },
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn socket_path(mut self, socket_path: impl Into<String>) -> Self {
        self.config.socket_uri = Some(socket_path.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.config.token = Some(token.into());
        self
    }

    pub fn client_name(mut self, client_name: impl Into<String>) -> Self {
        self.config.client_name = Some(client_name.into());
        self
    }

    pub async fn connect(self) -> Result<KiCadClient, KiCadError> {
        let socket_uri = resolve_socket_uri(self.config.socket_uri.as_deref());
        if is_missing_ipc_socket(&socket_uri) {
            return Err(KiCadError::SocketUnavailable { socket_uri });
        }

        let timeout = self.config.timeout;
        let transport = Transport::connect(&socket_uri, timeout)?;

        let token = self
            .config
            .token
            .or_else(|| std::env::var(KICAD_API_TOKEN_ENV).ok())
            .unwrap_or_default();

        let client_name = self
            .config
            .client_name
            .unwrap_or_else(default_client_name);

        Ok(KiCadClient {
            inner: Arc::new(ClientInner {
                transport,
                token: Mutex::new(token),
                client_name,
                timeout,
                socket_uri,
            }),
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl KiCadClient {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn connect() -> Result<Self, KiCadError> {
        ClientBuilder::new().connect().await
    }

    pub fn timeout(&self) -> Duration {
        self.inner.timeout
    }

    pub fn socket_uri(&self) -> &str {
        &self.inner.socket_uri
    }

    pub async fn ping(&self) -> Result<(), KiCadError> {
        let command = envelope::pack_any(&common_commands::Ping {}, CMD_PING);
        self.send_command(command).await?;
        Ok(())
    }

    pub async fn get_version(&self) -> Result<VersionInfo, KiCadError> {
        let command = envelope::pack_any(&common_commands::GetVersion {}, CMD_GET_VERSION);
        let response = self.send_command(command).await?;

        let payload: common_commands::GetVersionResponse =
            envelope::unpack_any(&response, RES_GET_VERSION)?;

        let version = payload
            .version
            .ok_or_else(|| KiCadError::MissingPayload {
                expected_type_url: "kiapi.common.types.KiCadVersion".to_string(),
            })?;

        Ok(VersionInfo {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            full_version: version.full_version,
        })
    }

    pub async fn get_open_documents(
        &self,
        document_type: DocumentType,
    ) -> Result<Vec<DocumentSpecifier>, KiCadError> {
        let command = common_commands::GetOpenDocuments {
            r#type: document_type.to_proto(),
        };

        let response = self
            .send_command(envelope::pack_any(&command, CMD_GET_OPEN_DOCUMENTS))
            .await?;

        let payload: common_commands::GetOpenDocumentsResponse =
            envelope::unpack_any(&response, RES_GET_OPEN_DOCUMENTS)?;

        Ok(payload
            .documents
            .into_iter()
            .filter_map(map_document_specifier)
            .collect())
    }

    pub async fn get_current_project_path(&self) -> Result<PathBuf, KiCadError> {
        let docs = self.get_open_documents(DocumentType::Pcb).await?;
        select_single_project_path(&docs)
    }

    pub async fn has_open_board(&self) -> Result<bool, KiCadError> {
        let docs = self.get_open_documents(DocumentType::Pcb).await?;
        Ok(!docs.is_empty())
    }

    async fn send_command(
        &self,
        command: prost_types::Any,
    ) -> Result<crate::proto::kiapi::common::ApiResponse, KiCadError> {
        let token = self
            .inner
            .token
            .lock()
            .map_err(|_| KiCadError::InternalPoisoned)?
            .clone();

        let request_bytes = envelope::encode_request(&token, &self.inner.client_name, command)?;

        let transport = self.inner.clone();
        let response_bytes = tokio::task::spawn_blocking(move || {
            transport.transport.roundtrip(request_bytes.as_slice())
        })
        .await
        .map_err(|err| KiCadError::RuntimeJoin(err.to_string()))??;

        let response = envelope::decode_response(&response_bytes)?;

        if let Some(err) = envelope::status_error(&response) {
            return Err(err);
        }

        if token.is_empty() {
            if let Some(header) = response.header.as_ref() {
                if !header.kicad_token.is_empty() {
                    let mut guard = self
                        .inner
                        .token
                        .lock()
                        .map_err(|_| KiCadError::InternalPoisoned)?;
                    *guard = header.kicad_token.clone();
                }
            }
        }

        Ok(response)
    }
}

fn map_document_specifier(source: common_types::DocumentSpecifier) -> Option<DocumentSpecifier> {
    let document_type = DocumentType::from_proto(source.r#type)?;
    let board_filename = match source.identifier {
        Some(common_types::document_specifier::Identifier::BoardFilename(filename)) => {
            Some(filename)
        }
        _ => None,
    };

    let project = source.project.unwrap_or_default();

    let project_info = ProjectInfo {
        name: if project.name.is_empty() {
            None
        } else {
            Some(project.name)
        },
        path: if project.path.is_empty() {
            None
        } else {
            Some(PathBuf::from(project.path))
        },
    };

    Some(DocumentSpecifier {
        document_type,
        board_filename,
        project: project_info,
    })
}

fn select_single_project_path(docs: &[DocumentSpecifier]) -> Result<PathBuf, KiCadError> {
    let mut paths = BTreeSet::new();
    for doc in docs {
        if let Some(path) = doc.project.path.as_ref() {
            paths.insert(path.display().to_string());
        }
    }

    if paths.is_empty() {
        return Err(KiCadError::BoardNotOpen);
    }

    if paths.len() > 1 {
        return Err(KiCadError::AmbiguousProjectPath {
            paths: paths.into_iter().collect(),
        });
    }

    let first = paths.into_iter().next().ok_or(KiCadError::BoardNotOpen)?;
    Ok(PathBuf::from(first))
}

fn resolve_socket_uri(explicit: Option<&str>) -> String {
    if let Some(socket) = explicit {
        return normalize_socket_uri(socket);
    }

    if let Ok(socket) = std::env::var(KICAD_API_SOCKET_ENV) {
        if !socket.is_empty() {
            return normalize_socket_uri(&socket);
        }
    }

    normalize_socket_uri(default_socket_path().to_string_lossy().as_ref())
}

fn default_socket_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return std::env::temp_dir().join("kicad").join("api.sock");
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(home) = std::env::var_os("HOME") {
            let flatpak = PathBuf::from(home)
                .join(".var")
                .join("app")
                .join("org.kicad.KiCad")
                .join("cache")
                .join("tmp")
                .join("kicad")
                .join("api.sock");
            if flatpak.exists() {
                return flatpak;
            }
        }

        PathBuf::from("/tmp/kicad/api.sock")
    }
}

fn normalize_socket_uri(socket: &str) -> String {
    if socket.contains("://") {
        return socket.to_string();
    }

    format!("ipc://{socket}")
}

fn ipc_path_from_uri(socket_uri: &str) -> Option<PathBuf> {
    let raw_path = socket_uri.strip_prefix("ipc://")?;
    Some(PathBuf::from(raw_path))
}

fn is_missing_ipc_socket(socket_uri: &str) -> bool {
    if let Some(path) = ipc_path_from_uri(socket_uri) {
        return !path.exists();
    }

    false
}

fn default_client_name() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);

    format!("kicad-ipc-{}-{millis}", std::process::id())
}

#[cfg(test)]
mod tests {
    use super::{normalize_socket_uri, select_single_project_path};
    use crate::error::KiCadError;
    use crate::model::common::{DocumentSpecifier, DocumentType, ProjectInfo};
    use std::path::PathBuf;

    #[test]
    fn normalize_socket_uri_adds_ipc_scheme() {
        let normalized = normalize_socket_uri("/tmp/kicad/api.sock");
        assert_eq!(normalized, "ipc:///tmp/kicad/api.sock");
    }

    #[test]
    fn normalize_socket_uri_preserves_existing_scheme() {
        let normalized = normalize_socket_uri("ipc:///tmp/kicad/api.sock");
        assert_eq!(normalized, "ipc:///tmp/kicad/api.sock");
    }

    #[test]
    fn select_single_project_path_picks_unique_path() {
        let docs = vec![DocumentSpecifier {
            document_type: DocumentType::Pcb,
            board_filename: Some("demo.kicad_pcb".to_string()),
            project: ProjectInfo {
                name: Some("demo".to_string()),
                path: Some(PathBuf::from("/tmp/demo")),
            },
        }];

        let result = select_single_project_path(&docs)
            .expect("a single project path should be selected when exactly one path exists");
        assert_eq!(result, PathBuf::from("/tmp/demo"));
    }

    #[test]
    fn select_single_project_path_errors_on_ambiguity() {
        let docs = vec![
            DocumentSpecifier {
                document_type: DocumentType::Pcb,
                board_filename: Some("a.kicad_pcb".to_string()),
                project: ProjectInfo {
                    name: Some("a".to_string()),
                    path: Some(PathBuf::from("/tmp/a")),
                },
            },
            DocumentSpecifier {
                document_type: DocumentType::Pcb,
                board_filename: Some("b.kicad_pcb".to_string()),
                project: ProjectInfo {
                    name: Some("b".to_string()),
                    path: Some(PathBuf::from("/tmp/b")),
                },
            },
        ];

        let result = select_single_project_path(&docs);
        assert!(matches!(
            result,
            Err(KiCadError::AmbiguousProjectPath { .. })
        ));
    }

    #[test]
    fn select_single_project_path_requires_open_board() {
        let docs: Vec<DocumentSpecifier> = Vec::new();
        let result = select_single_project_path(&docs);
        assert!(matches!(result, Err(KiCadError::BoardNotOpen)));
    }
}
