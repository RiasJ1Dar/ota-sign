use clap::{Parser, Subcommand};
use ota_sign::{apply_from_dir, apply_from_url, generate_keypair, publish_dir};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "ota-sign", about = "Publish and apply Ed25519-signed OTA blobs")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Write `name.secret` + `name.public` (hex Ed25519).
    Keygen {
        /// Path prefix, e.g. `./keys/release` → `release.secret` / `release.public`.
        #[arg(value_name = "PREFIX")]
        prefix: PathBuf,
    },
    /// Hash a directory into `out/` (manifest + blobs) and sign it.
    Publish {
        /// Directory of files to ship.
        #[arg(long)]
        source: PathBuf,
        /// Output channel directory.
        #[arg(long)]
        out: PathBuf,
        /// App id stored in the manifest.
        #[arg(long)]
        app: String,
        /// Version string stored in the manifest.
        #[arg(long)]
        version: String,
        /// Path to `*.secret` hex key.
        #[arg(long)]
        secret: PathBuf,
    },
    /// Verify + apply a local channel directory into install dir.
    Apply {
        /// Channel dir with manifest.json + blobs/.
        #[arg(long)]
        channel: PathBuf,
        /// Where to write files.
        #[arg(long)]
        install: PathBuf,
        /// Path to `*.public` hex key.
        #[arg(long)]
        public: PathBuf,
    },
    /// Same as apply, but fetch from HTTP(S) base URL.
    #[command(name = "apply-url")]
    ApplyUrl {
        /// Base URL (no trailing slash required).
        #[arg(long)]
        base: String,
        #[arg(long)]
        install: PathBuf,
        #[arg(long)]
        public: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let res = match cli.cmd {
        Cmd::Keygen { prefix } => generate_keypair(&prefix).map(|k| {
            println!("secret: {}", k.secret_path.display());
            println!("public: {}", k.public_path.display());
        }),
        Cmd::Publish {
            source,
            out,
            app,
            version,
            secret,
        } => publish_dir(&source, &out, &app, &version, &secret).map(|m| {
            println!(
                "published {} {} ({} files)",
                m.app,
                m.version,
                m.files.len()
            );
        }),
        Cmd::Apply {
            channel,
            install,
            public,
        } => apply_from_dir(&channel, &install, &public).map(|r| {
            println!(
                "applied {}: updated {}, skipped {}",
                r.version, r.updated, r.skipped
            );
        }),
        Cmd::ApplyUrl {
            base,
            install,
            public,
        } => apply_from_url(&base, &install, &public).map(|r| {
            println!(
                "applied {}: updated {}, skipped {}",
                r.version, r.updated, r.skipped
            );
        }),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
