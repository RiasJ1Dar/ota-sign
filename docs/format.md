# ota-sign format 1

Layout of a published update channel (GitHub repo, static HTTP, USB stick):

```
manifest.json      # cleartext file list + version
manifest.sig       # Ed25519 signature over the raw bytes of manifest.json
blobs/<sha256>.bin # one blob per unique file content (lowercase hex name)
```

## manifest.json

```json
{
  "format": 1,
  "app": "MyApp",
  "version": "1.2.0",
  "files": [
    { "path": "MyApp.exe", "sha256": "...", "size": 12345 }
  ]
}
```

- `path` — relative, `/`-separated, no `..`.
- `sha256` — lowercase hex of the **plaintext** file.
- Blobs are **not encrypted** in v1 (authenticity via signature). Obfuscation
  like szi-as-ota can be a later optional layer; this kit optimizes for open apps.

## Verification

1. Fetch `manifest.json` + `manifest.sig`.
2. Verify Ed25519 with the app’s embedded public key.
3. For each file: if local sha256 matches, skip; else download `blobs/<sha256>.bin`,
   check hash, write atomically.
