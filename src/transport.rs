use std::sync::Mutex;
use std::time::Duration;

use nng::options::{Options, RecvTimeout, SendTimeout};
use nng::{Error as NngError, Protocol, Socket};

use crate::error::KiCadError;

#[derive(Debug)]
pub(crate) struct Transport {
    socket: Mutex<Socket>,
    timeout: Duration,
}

impl Transport {
    pub(crate) fn connect(socket_uri: &str, timeout: Duration) -> Result<Self, KiCadError> {
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

        socket.dial(socket_uri).map_err(|err| KiCadError::Connection {
            socket_uri: socket_uri.to_string(),
            reason: err.to_string(),
        })?;

        Ok(Self {
            socket: Mutex::new(socket),
            timeout,
        })
    }

    pub(crate) fn roundtrip(&self, request_bytes: &[u8]) -> Result<Vec<u8>, KiCadError> {
        let guard = self
            .socket
            .lock()
            .map_err(|_| KiCadError::InternalPoisoned)?;

        guard
            .send(request_bytes)
            .map_err(|(_, err)| map_send_error(err, self.timeout))?;

        let response = guard
            .recv()
            .map_err(|err| map_receive_error(err, self.timeout))?;

        Ok(response.as_slice().to_vec())
    }
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
