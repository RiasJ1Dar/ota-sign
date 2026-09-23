//! Incremental OTA: signed manifest + content-addressed blobs.

#![deny(missing_docs)]

mod apply;
mod error;
mod keys;
mod manifest;
mod publish;

pub use apply::{apply_from_dir, apply_from_url, ApplyReport};
pub use error::Error;
pub use keys::{generate_keypair, load_public_key, load_signing_key, KeypairFiles};
pub use manifest::{FileEntry, Manifest};
pub use publish::publish_dir;
