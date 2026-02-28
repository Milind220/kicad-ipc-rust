use std::time::Duration;

use thiserror::Error;

#[derive(Debug, Error)]
/// Error type returned by `kicad-ipc-rs` operations.
pub enum KiCadError {
    /// Invalid local configuration or user input before IPC dispatch.
    #[error("invalid configuration: {reason}")]
    Config { reason: String },

    /// KiCad IPC socket could not be found at connect time.
    #[error("KiCad IPC socket not available at `{socket_uri}`. Open KiCad and open a project/board first.")]
    SocketUnavailable { socket_uri: String },

    /// IPC connection failed.
    #[error("connection failed for `{socket_uri}`: {reason}")]
    Connection { socket_uri: String, reason: String },

    /// Transport send path failed.
    #[error("transport send failed: {reason}")]
    TransportSend { reason: String },

    /// Transport receive path failed.
    #[error("transport receive failed: {reason}")]
    TransportReceive { reason: String },

    /// Background transport task has stopped.
    #[error("transport task is unavailable")]
    TransportClosed,

    /// Request exceeded configured timeout.
    #[error("request timed out after {timeout:?}")]
    Timeout { timeout: Duration },

    /// KiCad returned a non-success API status.
    #[error("API status error `{code}`: {message}")]
    ApiStatus { code: String, message: String },

    /// KiCad returned a non-success per-item status.
    #[error("item request status error `{code}`")]
    ItemStatus { code: String },

    /// Response payload content was malformed or inconsistent.
    #[error("invalid API response: {reason}")]
    InvalidResponse { reason: String },

    /// Response payload was missing when required.
    #[error("API response missing payload for `{expected_type_url}`")]
    MissingPayload { expected_type_url: String },

    /// Response payload type did not match expected protobuf type URL.
    #[error("unexpected payload type; expected `{expected_type_url}`, got `{actual_type_url}`")]
    UnexpectedPayloadType {
        expected_type_url: String,
        actual_type_url: String,
    },

    /// Protobuf encoding failed.
    #[error("protobuf encode failed: {0}")]
    ProtobufEncode(String),

    /// Protobuf decoding failed.
    #[error("protobuf decode failed: {0}")]
    ProtobufDecode(String),

    /// Blocking runtime worker join failed.
    #[error("runtime task join failed: {0}")]
    RuntimeJoin(String),

    /// Blocking runtime worker is unavailable.
    #[error("blocking runtime is unavailable")]
    BlockingRuntimeClosed,

    /// Internal mutex poisoning detected.
    #[error("mutex poisoned")]
    InternalPoisoned,

    /// Operation requires an open PCB document.
    #[error("no open PCB document found; open a board in KiCad first")]
    BoardNotOpen,

    /// Multiple project paths were detected where a single path was required.
    #[error("multiple project paths found across open PCB docs: {paths:?}")]
    AmbiguousProjectPath { paths: Vec<String> },

    /// Multiple open PCB docs prevent choosing an implicit board context.
    #[error("multiple PCB documents are open; unable to choose one board context: {boards:?}")]
    AmbiguousBoardSelection { boards: Vec<String> },
}
