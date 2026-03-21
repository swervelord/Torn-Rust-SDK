use thiserror::Error;

use crate::client::ClientError;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error type returned by high-level wrapper APIs.
pub enum SdkError {
    /// Wrapped low-level client failure.
    #[error("client error: {0}")]
    Client(#[from] ClientError),
    /// Typed decoding failure for wrapper model payloads.
    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),
    /// Client-side validation failure prior to network dispatch.
    #[error("validation error: {0}")]
    Validation(String),
}
