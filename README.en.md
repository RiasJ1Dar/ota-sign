[Українська](README.md) · **English**

# ota-sign

Open incremental OTA: Ed25519-signed manifest + SHA-512 content-addressed blobs.
Publisher CLI and embeddable client library.

Inspired by the *download only what changed* idea behind `szi-as-ota`, without the
proprietary SZIU crypto — v1 is intentionally cleartext; authenticity is the signature.

See [docs/format.md](docs/format.md).

```bash
ota-sign keygen ./keys/release
ota-sign publish --source ./dist --out ./channel --app MyApp --version 1.2.0 --secret ./keys/release.secret
ota-sign apply --channel ./channel --install ./app --public ./keys/release.public
```

## License

MIT
