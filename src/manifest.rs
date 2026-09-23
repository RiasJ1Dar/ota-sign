use crate::Error;
use serde::{Deserialize, Serialize};

/// One file in the install tree.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileEntry {
    /// Relative path using `/`, no `..`.
    pub path: String,
    /// Lowercase hex SHA-512 of plaintext.
    pub sha512: String,
    /// Size in bytes.
    pub size: u64,
}

/// Signed update manifest (format 2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    /// Format version; must be `2`.
    pub format: u32,
    /// Application id (informational).
    pub app: String,
    /// Version string (informational / for UI).
    pub version: String,
    /// Files to materialize.
    pub files: Vec<FileEntry>,
}

impl Manifest {
    /// Validate format and paths.
    pub fn validate(&self) -> Result<(), Error> {
        if self.format != 2 {
            return Err(Error::Invalid(format!(
                "unsupported format {}",
                self.format
            )));
        }
        if self.app.trim().is_empty() || self.version.trim().is_empty() {
            return Err(Error::Invalid("app/version required".into()));
        }
        for f in &self.files {
            validate_path(&f.path)?;
            if f.sha512.len() != 128 || !f.sha512.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(Error::Invalid(format!("bad sha512 for {}", f.path)));
            }
        }
        Ok(())
    }

    /// Canonical JSON bytes used for signing (sorted keys via serde default order
    /// of struct fields — keep field order stable).
    pub fn to_sign_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec_pretty(self)?)
    }
}

fn validate_path(path: &str) -> Result<(), Error> {
    if path.is_empty() || path.starts_with('/') || path.contains('\\') {
        return Err(Error::Invalid(format!("bad path {path}")));
    }
    for part in path.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return Err(Error::Invalid(format!("bad path {path}")));
        }
    }
    Ok(())
}
