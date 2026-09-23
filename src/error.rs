use thiserror::Error;

/// Crate-level error.
#[derive(Debug, Error)]
pub enum Error {
    /// I/O failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// JSON encode/decode.
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    /// Cryptographic / signature failure.
    #[error("crypto: {0}")]
    Crypto(String),
    /// Manifest or path failed validation.
    #[error("invalid manifest: {0}")]
    Invalid(String),
    /// HTTP fetch failed.
    #[error("http: {0}")]
    Http(String),
    /// Blob hash mismatch after download.
    #[error("hash mismatch for {path}: expected {expected}, got {got}")]
    HashMismatch {
        /// Relative path in the install tree.
        path: String,
        /// Expected sha512 hex.
        expected: String,
        /// Actual sha512 hex.
        got: String,
    },
}
