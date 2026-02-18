use std::thread;
use std::time::Duration;

use nng::options::{Options, RecvTimeout, SendTimeout};
use nng::{Error as NngError, Protocol, Socket};
use tokio::sync::{mpsc, oneshot};

use crate::error::KiCadError;

const TRANSPORT_QUEUE_CAPACITY: usize = 64;

#[derive(Debug)]
pub(crate) struct Transport {
    request_tx: mpsc::Sender<TransportRequest>,
}

#[derive(Debug)]
struct TransportRequest {
    request_bytes: Vec<u8>,
    response_tx: oneshot::Sender<Result<Vec<u8>, KiCadError>>,
}

impl Transport {
    pub(crate) fn connect(socket_uri: &str, timeout: Duration) -> Result<Self, KiCadError> {
        let socket = configured_socket(socket_uri, timeout)?;
        let (request_tx, mut request_rx) =
            mpsc::channel::<TransportRequest>(TRANSPORT_QUEUE_CAPACITY);

        let worker_name = format!("kicad-ipc-transport-{}", std::process::id());
        thread::Builder::new()
            .name(worker_name)
            .spawn(move || {
                while let Some(request) = request_rx.blocking_recv() {
                    let response =
                        socket_roundtrip(&socket, request.request_bytes.as_slice(), timeout);
                    let _ = request.response_tx.send(response);
                }
            })
            .map_err(|err| KiCadError::Connection {
                socket_uri: socket_uri.to_string(),
                reason: err.to_string(),
            })?;

        Ok(Self { request_tx })
    }

    pub(crate) async fn roundtrip(&self, request_bytes: Vec<u8>) -> Result<Vec<u8>, KiCadError> {
        let (response_tx, response_rx) = oneshot::channel();

        self.request_tx
            .send(TransportRequest {
                request_bytes,
                response_tx,
            })
            .await
            .map_err(|_| KiCadError::TransportClosed)?;

        response_rx.await.map_err(|_| KiCadError::TransportClosed)?
    }
}

fn configured_socket(socket_uri: &str, timeout: Duration) -> Result<Socket, KiCadError> {
    let socket = Socket::new(Protocol::Req0).map_err(|err| KiCadError::Connection {
        socket_uri: socket_uri.to_string(),
        reason: err.to_string(),
    })?;

    socket
        .set_opt::<SendTimeout>(Some(timeout))
        .map_err(|err| KiCadError::Connection {
            socket_uri: socket_uri.to_string(),
            reason: err.to_string(),
        })?;

    socket
        .set_opt::<RecvTimeout>(Some(timeout))
        .map_err(|err| KiCadError::Connection {
            socket_uri: socket_uri.to_string(),
            reason: err.to_string(),
        })?;

    socket
        .dial(socket_uri)
        .map_err(|err| KiCadError::Connection {
            socket_uri: socket_uri.to_string(),
            reason: err.to_string(),
        })?;

    Ok(socket)
}

fn socket_roundtrip(
    socket: &Socket,
    request_bytes: &[u8],
    timeout: Duration,
) -> Result<Vec<u8>, KiCadError> {
    socket
        .send(request_bytes)
        .map_err(|(_, err)| map_send_error(err, timeout))?;

    let response = socket
        .recv()
        .map_err(|err| map_receive_error(err, timeout))?;

    Ok(response.as_slice().to_vec())
}

fn map_send_error(error: NngError, timeout: Duration) -> KiCadError {
    if error == NngError::TimedOut {
        return KiCadError::Timeout { timeout };
    }

    KiCadError::TransportSend {
        reason: error.to_string(),
    }
}

fn map_receive_error(error: NngError, timeout: Duration) -> KiCadError {
    if error == NngError::TimedOut {
        return KiCadError::Timeout { timeout };
    }

    KiCadError::TransportReceive {
        reason: error.to_string(),
    }
}
