# ota-sign format 2

Layout of a published update channel (GitHub repo, static HTTP, USB stick):

```
manifest.json      # cleartext file list + version
manifest.sig       # Ed25519 signature over the raw bytes of manifest.json
blobs/<sha512>.bin # one blob per unique file content (lowercase hex name)
```

## manifest.json

```json
{
  "format": 2,
  "app": "MyApp",
  "version": "1.2.0",
  "files": [
    { "path": "MyApp.exe", "sha512": "...", "size": 12345 }
  ]
}
```

- `path` — relative, `/`-separated, no `..`.
- `sha512` — lowercase hex of the **plaintext** file.
- Blobs are **not encrypted** in v1 (authenticity via signature). Obfuscation
  like szi-as-ota can be a later optional layer; this kit optimizes for open apps.

## Verification

1. Fetch `manifest.json` + `manifest.sig`.
2. Verify Ed25519 with the app’s embedded public key.
3. For each file: if local sha512 matches, skip; else download `blobs/<sha512>.bin`,
   check hash, write atomically.
