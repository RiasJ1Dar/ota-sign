use crate::keys::load_public_key;
use crate::manifest::Manifest;
use crate::Error;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Summary of an apply run.
#[derive(Debug, Default, Clone)]
pub struct ApplyReport {
    /// Files already matching the manifest.
    pub skipped: usize,
    /// Files downloaded or copied.
    pub updated: usize,
    /// Manifest version string.
    pub version: String,
}

/// Apply an update channel from a local directory (`manifest.json` + `blobs/`).
pub fn apply_from_dir(
    channel_dir: &Path,
    install_dir: &Path,
    public_key_path: &Path,
) -> Result<ApplyReport, Error> {
    let pk = load_public_key(public_key_path)?;
    let body = fs::read(channel_dir.join("manifest.json"))?;
    let sig_hex = fs::read_to_string(channel_dir.join("manifest.sig"))?;
    verify_manifest(&body, sig_hex.trim(), &pk)?;
    let manifest: Manifest = serde_json::from_slice(&body)?;
    manifest.validate()?;
    materialize(&manifest, install_dir, |hash| {
        let p = channel_dir.join("blobs").join(format!("{hash}.bin"));
        Ok(fs::read(p)?)
    })
}

/// Apply from HTTP(S) base URL (`{base}/manifest.json`, `{base}/blobs/...`).
pub fn apply_from_url(
    base_url: &str,
    install_dir: &Path,
    public_key_path: &Path,
) -> Result<ApplyReport, Error> {
    let pk = load_public_key(public_key_path)?;
    let base = base_url.trim_end_matches('/');
    let body = http_get_bytes(&format!("{base}/manifest.json"))?;
    let sig_hex = String::from_utf8(http_get_bytes(&format!("{base}/manifest.sig"))?)
        .map_err(|e| Error::Invalid(e.to_string()))?;
    verify_manifest(&body, sig_hex.trim(), &pk)?;
    let manifest: Manifest = serde_json::from_slice(&body)?;
    manifest.validate()?;
    materialize(&manifest, install_dir, |hash| {
        http_get_bytes(&format!("{base}/blobs/{hash}.bin"))
    })
}

fn verify_manifest(body: &[u8], sig_hex: &str, pk: &VerifyingKey) -> Result<(), Error> {
    let sig_bytes = hex::decode(sig_hex).map_err(|e| Error::Crypto(e.to_string()))?;
    let sig = Signature::from_slice(&sig_bytes).map_err(|e| Error::Crypto(e.to_string()))?;
    pk.verify(body, &sig)
        .map_err(|e| Error::Crypto(format!("signature: {e}")))?;
    Ok(())
}

fn materialize<F>(
    manifest: &Manifest,
    install_dir: &Path,
    mut fetch: F,
) -> Result<ApplyReport, Error>
where
    F: FnMut(&str) -> Result<Vec<u8>, Error>,
{
    fs::create_dir_all(install_dir)?;
    let mut report = ApplyReport {
        version: manifest.version.clone(),
        ..Default::default()
    };

    for f in &manifest.files {
        let dest = safe_join(install_dir, &f.path)?;
        if dest.is_file() {
            let mut cur = Vec::new();
            fs::File::open(&dest)?.read_to_end(&mut cur)?;
            let got = hex::encode(Sha256::digest(&cur));
            if got == f.sha256 {
                report.skipped += 1;
                continue;
            }
        }
        let data = fetch(&f.sha256)?;
        let got = hex::encode(Sha256::digest(&data));
        if got != f.sha256 {
            return Err(Error::HashMismatch {
                path: f.path.clone(),
                expected: f.sha256.clone(),
                got,
            });
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp = dest.with_extension("ota-tmp");
        fs::write(&tmp, &data)?;
        fs::rename(&tmp, &dest)?;
        report.updated += 1;
    }
    Ok(report)
}

fn safe_join(base: &Path, rel: &str) -> Result<PathBuf, Error> {
    let mut out = base.to_path_buf();
    for part in rel.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return Err(Error::Invalid(format!("bad path {rel}")));
        }
        out.push(part);
    }
    Ok(out)
}

fn http_get_bytes(url: &str) -> Result<Vec<u8>, Error> {
    let resp = ureq::get(url)
        .set("User-Agent", "ota-sign/0.1")
        .call()
        .map_err(|e| Error::Http(e.to_string()))?;
    let mut buf = Vec::new();
    resp.into_reader()
        .read_to_end(&mut buf)
        .map_err(|e| Error::Http(e.to_string()))?;
    Ok(buf)
}
