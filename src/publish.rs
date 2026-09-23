use crate::keys::load_signing_key;
use crate::manifest::{FileEntry, Manifest};
use crate::Error;
use ed25519_dalek::Signer;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Walk `source_dir`, write blobs + signed manifest into `out_dir`.
pub fn publish_dir(
    source_dir: &Path,
    out_dir: &Path,
    app: &str,
    version: &str,
    secret_key_path: &Path,
) -> Result<Manifest, Error> {
    let signing = load_signing_key(secret_key_path)?;
    let blobs = out_dir.join("blobs");
    fs::create_dir_all(&blobs)?;

    let mut files = Vec::new();
    collect_files(source_dir, source_dir, &mut files)?;

    let mut entries = Vec::new();
    for abs in files {
        let rel = abs
            .strip_prefix(source_dir)
            .map_err(|_| Error::Invalid("path prefix".into()))?;
        let rel_s = rel.to_string_lossy().replace('\\', "/");
        let mut data = Vec::new();
        fs::File::open(&abs)?.read_to_end(&mut data)?;
        let hash = hex::encode(Sha256::digest(&data));
        let size = data.len() as u64;
        let blob = blobs.join(format!("{hash}.bin"));
        if !blob.exists() {
            fs::write(&blob, &data)?;
        }
        entries.push(FileEntry {
            path: rel_s,
            sha256: hash,
            size,
        });
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));

    let manifest = Manifest {
        format: 1,
        app: app.to_string(),
        version: version.to_string(),
        files: entries,
    };
    manifest.validate()?;

    let body = manifest.to_sign_bytes()?;
    let sig = signing.sign(&body);
    fs::write(out_dir.join("manifest.json"), &body)?;
    fs::write(
        out_dir.join("manifest.sig"),
        hex::encode(sig.to_bytes()),
    )?;
    Ok(manifest)
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), Error> {
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let path = ent.path();
        let name = ent.file_name();
        let name_s = name.to_string_lossy();
        if name_s.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            collect_files(root, &path, out)?;
        } else if path.is_file() {
            out.push(path);
        }
    }
    Ok(())
}
