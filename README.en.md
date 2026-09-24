[Українська](README.md) · **English**

# ota-sign

An open incremental OTA format: an Ed25519-signed manifest plus SHA-512
content-addressed blobs. The repository provides a publisher CLI and a Rust
library for embedding the client side in an application.

The idea comes from `szi-as-ota`: transfer only files that changed. Format 2
is intentionally transparent. It provides no confidentiality or obfuscation;
the signature authenticates the manifest and SHA-512 verifies each file.

## Requirements and installation

A Rust toolchain with Cargo is required.

```bash
cargo install --git https://github.com/RiasJ1Dar/ota-sign
```

Or install from a clone:

```bash
git clone https://github.com/RiasJ1Dar/ota-sign.git
cd ota-sign
cargo install --path .
```

## Create the first update channel

Generate a keypair once:

```bash
ota-sign keygen ./keys/release
```

This creates `release.secret` and `release.public`. Keep the secret key only
in the publishing environment and never commit `*.secret`. Ship or embed the
public key with the client.

Publish the contents of `dist/`:

```bash
ota-sign publish \
  --source ./dist \
  --out ./channel \
  --app MyApp \
  --version 1.2.0 \
  --secret ./keys/release.secret
```

Output:

```text
channel/
├── manifest.json
├── manifest.sig
└── blobs/
    └── <sha512>.bin
```

Files with identical content share one blob. Hidden files and directories whose
names begin with a dot are skipped during publishing.

## Apply an update

From a local channel:

```bash
ota-sign apply \
  --channel ./channel \
  --install ./app \
  --public ./keys/release.public
```

From static HTTP, GitHub Raw, or Pages:

```bash
ota-sign apply-url \
  --base https://raw.githubusercontent.com/You/my-ota/main \
  --install ./app \
  --public ./keys/release.public
```

The client verifies the manifest signature first. For each entry, it skips a
local file whose SHA-512 already matches; otherwise it fetches the blob, checks
its hash, and replaces the destination through a temporary `.ota-tmp` file.

Files absent from the manifest are not deleted. This is a delivery and
materialization format, not a full directory synchronizer.

## Commands

| Command | Purpose |
|---|---|
| `keygen PREFIX` | Write `PREFIX.secret` and `PREFIX.public` as hex |
| `publish --source DIR --out DIR --app ID --version VER --secret PATH` | Build and sign a channel |
| `apply --channel DIR --install DIR --public PATH` | Verify and apply a local channel |
| `apply-url --base URL --install DIR --public PATH` | Fetch and apply an HTTP(S) channel |

`apply` and `apply-url` print the version and the updated/skipped file counts.

## Host integration

```rust
use ota_sign::apply_from_url;
use std::path::Path;

let report = apply_from_url(
    "https://example.com/channel",
    Path::new("./app"),
    Path::new("./release.public"),
)?;

println!(
    "version {}, updated {}, skipped {}",
    report.version, report.updated, report.skipped
);
```

The public API also exposes `apply_from_dir`, `publish_dir`,
`generate_keypair`, `Manifest`, `FileEntry`, and key-loading helpers.

## Format and guarantees

See [docs/format.md](docs/format.md) for the full format.

- `manifest.sig` is an Ed25519 signature over the raw `manifest.json` bytes;
- manifest paths are relative and `/`-separated, with no empty, `.`, or `..`
  components;
- each blob name is the lowercase SHA-512 of its cleartext content;
- the client verifies both the signature and each blob hash before writing;
- blobs are not encrypted;
- the channel format does not perform Windows Authenticode signing.

Used with [desktop-remote-kit](https://github.com/RiasJ1Dar/desktop-remote-kit),
the kit detects a newer GitHub Release and `ota-sign` delivers its files.
[kit-packager](https://github.com/RiasJ1Dar/kit-packager) automates build,
channel publishing, and GitHub Release creation.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

MIT