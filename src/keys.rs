use crate::Error;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::{Path, PathBuf};

/// Paths written by [`generate_keypair`].
#[derive(Debug, Clone)]
pub struct KeypairFiles {
    /// Hex-encoded 32-byte secret seed.
    pub secret_path: PathBuf,
    /// Hex-encoded 32-byte public key.
    pub public_path: PathBuf,
}

/// Write a new Ed25519 keypair as hex files (`*.secret`, `*.public`).
pub fn generate_keypair(prefix: &Path) -> Result<KeypairFiles, Error> {
    let signing = SigningKey::generate(&mut OsRng);
    let verifying = signing.verifying_key();
    let secret_path = prefix.with_extension("secret");
    let public_path = prefix.with_extension("public");
    fs::write(&secret_path, hex::encode(signing.to_bytes()))?;
    fs::write(&public_path, hex::encode(verifying.to_bytes()))?;
    Ok(KeypairFiles {
        secret_path,
        public_path,
    })
}

/// Load signing key from hex file (64 hex chars = 32 bytes).
pub fn load_signing_key(path: &Path) -> Result<SigningKey, Error> {
    let raw = fs::read_to_string(path)?;
    let bytes = hex::decode(raw.trim()).map_err(|e| Error::Crypto(e.to_string()))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| Error::Crypto("signing key must be 32 bytes".into()))?;
    Ok(SigningKey::from_bytes(&arr))
}

/// Load public key from hex file.
pub fn load_public_key(path: &Path) -> Result<VerifyingKey, Error> {
    let raw = fs::read_to_string(path)?;
    let bytes = hex::decode(raw.trim()).map_err(|e| Error::Crypto(e.to_string()))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| Error::Crypto("public key must be 32 bytes".into()))?;
    VerifyingKey::from_bytes(&arr).map_err(|e| Error::Crypto(e.to_string()))
}
