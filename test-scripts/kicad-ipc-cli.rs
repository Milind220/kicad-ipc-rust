use std::process::ExitCode;
use std::str::FromStr;
use std::time::Duration;

use kicad_ipc::{ClientBuilder, DocumentType, KiCadError};

#[derive(Debug)]
struct CliConfig {
    socket: Option<String>,
    token: Option<String>,
    timeout_ms: u64,
}

#[derive(Debug)]
enum Command {
    Ping,
    Version,
    OpenDocs { document_type: DocumentType },
    ProjectPath,
    BoardOpen,
    Nets,
    EnabledLayers,
    ActiveLayer,
    Smoke,
    Help,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            if matches!(
                err,
                KiCadError::BoardNotOpen | KiCadError::SocketUnavailable { .. }
            ) {
                eprintln!(
                    "hint: launch KiCad, open a project, and open a PCB editor window before rerunning this command."
                );
            }
            if let KiCadError::ApiStatus { code, message } = &err {
                if code == "AS_UNHANDLED" {
                    eprintln!(
                        "hint: this KiCad build reported the command as unavailable (`{message}`). try `ping` and `version`, or update KiCad/API settings."
                    );
                }
            }
            ExitCode::from(1)
        }
    }
}

async fn run() -> Result<(), KiCadError> {
    let (config, command) = parse_args()?;

    if matches!(command, Command::Help) {
        print_help();
        return Ok(());
    }

    let mut builder = ClientBuilder::new().timeout(Duration::from_millis(config.timeout_ms));
    if let Some(socket) = config.socket {
        builder = builder.socket_path(socket);
    }
    if let Some(token) = config.token {
        builder = builder.token(token);
    }

    let client = builder.connect().await?;

    match command {
        Command::Ping => {
            client.ping().await?;
            println!("pong");
        }
        Command::Version => {
            let version = client.get_version().await?;
            println!(
                "version: {}.{}.{} ({})",
                version.major, version.minor, version.patch, version.full_version
            );
        }
        Command::OpenDocs { document_type } => {
            let docs = client.get_open_documents(document_type).await?;
            if docs.is_empty() {
                println!("no open `{document_type}` documents");
            } else {
                for (idx, doc) in docs.iter().enumerate() {
                    let board = doc.board_filename.as_deref().unwrap_or("-");
                    let project_name = doc.project.name.as_deref().unwrap_or("-");
                    let project_path = doc
                        .project
                        .path
                        .as_ref()
                        .map(|path| path.display().to_string())
                        .unwrap_or_else(|| "-".to_string());

                    println!(
                        "[{idx}] type={} board={} project_name={} project_path={}",
                        doc.document_type, board, project_name, project_path
                    );
                }
            }
        }
        Command::ProjectPath => {
            let path = client.get_current_project_path().await?;
            println!("project_path={}", path.display());
        }
        Command::BoardOpen => {
            let has_board = client.has_open_board().await?;
            if has_board {
                println!("board-open: yes");
            } else {
                return Err(KiCadError::BoardNotOpen);
            }
        }
        Command::Nets => {
            let nets = client.get_nets().await?;
            if nets.is_empty() {
                println!("no nets returned");
            } else {
                for net in nets {
                    println!("code={} name={}", net.code, net.name);
                }
            }
        }
        Command::EnabledLayers => {
            let enabled = client.get_board_enabled_layers().await?;
            println!("copper_layer_count={}", enabled.copper_layer_count);
            for layer in enabled.layers {
                println!("layer_id={} layer_name={}", layer.id, layer.name);
            }
        }
        Command::ActiveLayer => {
            let layer = client.get_active_layer().await?;
            println!(
                "active_layer_id={} active_layer_name={}",
                layer.id, layer.name
            );
        }
        Command::Smoke => {
            client.ping().await?;
            let version = client.get_version().await?;
            let has_board = client.has_open_board().await?;
            println!(
                "smoke ok: version={}.{}.{} board_open={}",
                version.major, version.minor, version.patch, has_board
            );
        }
        Command::Help => print_help(),
    }

    Ok(())
}

fn parse_args() -> Result<(CliConfig, Command), KiCadError> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        return Ok((default_config(), Command::Help));
    }

    let mut config = default_config();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--socket" => {
                let value = args.get(index + 1).ok_or_else(|| KiCadError::Config {
                    reason: "missing value for --socket".to_string(),
                })?;
                config.socket = Some(value.clone());
                args.drain(index..=index + 1);
            }
            "--token" => {
                let value = args.get(index + 1).ok_or_else(|| KiCadError::Config {
                    reason: "missing value for --token".to_string(),
                })?;
                config.token = Some(value.clone());
                args.drain(index..=index + 1);
            }
            "--timeout-ms" => {
                let value = args.get(index + 1).ok_or_else(|| KiCadError::Config {
                    reason: "missing value for --timeout-ms".to_string(),
                })?;
                config.timeout_ms = value.parse::<u64>().map_err(|err| KiCadError::Config {
                    reason: format!("invalid --timeout-ms value `{value}`: {err}"),
                })?;
                args.drain(index..=index + 1);
            }
            _ => {
                index += 1;
            }
        }
    }

    if args.is_empty() {
        return Ok((config, Command::Help));
    }

    let command = match args[0].as_str() {
        "help" | "--help" | "-h" => Command::Help,
        "ping" => Command::Ping,
        "version" => Command::Version,
        "project-path" => Command::ProjectPath,
        "board-open" => Command::BoardOpen,
        "nets" => Command::Nets,
        "enabled-layers" => Command::EnabledLayers,
        "active-layer" => Command::ActiveLayer,
        "smoke" => Command::Smoke,
        "open-docs" => {
            let mut document_type = DocumentType::Pcb;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--type" {
                    let value = args.get(i + 1).ok_or_else(|| KiCadError::Config {
                        reason: "missing value for open-docs --type".to_string(),
                    })?;
                    document_type = DocumentType::from_str(value)
                        .map_err(|err| KiCadError::Config { reason: err })?;
                    i += 2;
                    continue;
                }
                i += 1;
            }
            Command::OpenDocs { document_type }
        }
        other => {
            return Err(KiCadError::Config {
                reason: format!("unknown command `{other}`"),
            });
        }
    };

    Ok((config, command))
}

fn default_config() -> CliConfig {
    CliConfig {
        socket: None,
        token: None,
        timeout_ms: 3_000,
    }
}

fn print_help() {
    println!(
        "kicad-ipc-cli\n\nUSAGE:\n  cargo run --bin kicad-ipc-cli -- [--socket URI] [--token TOKEN] [--timeout-ms N] <command> [command options]\n\nCOMMANDS:\n  ping                         Check IPC connectivity\n  version                      Fetch KiCad version\n  open-docs [--type <type>]    List open docs (default type: pcb)\n  project-path                 Get current project path from open PCB docs\n  board-open                   Exit non-zero if no PCB doc is open\n  nets                         List board nets (requires one open PCB)\n  enabled-layers               List enabled board layers\n  active-layer                 Show active board layer\n  smoke                        ping + version + board-open summary\n  help                         Show help\n\nTYPES:\n  schematic | symbol | pcb | footprint | drawing-sheet | project\n"
    );
}
