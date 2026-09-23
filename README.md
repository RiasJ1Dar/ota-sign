**Українська** · [English](README.en.md)

# ota-sign

Відкритий інкрементний OTA: підписаний Ed25519 маніфест + блоби за SHA-512.
CLI для автора і бібліотека для вбудовування в `.exe`.

Натхненно ідеєю `szi-as-ota` (качати лише змінені файли), але **без** пропрієтарного
шифрування SZIU — v1 навмисне прозорий: автентичність дає підпис, не обфускація.

## Формат

Див. [docs/format.md](docs/format.md): `manifest.json` + `manifest.sig` + `blobs/<sha512>.bin`.

## CLI

```bash
cargo install --path .

ota-sign keygen ./keys/release
ota-sign publish --source ./dist --out ./channel --app MyApp --version 1.2.0 --secret ./keys/release.secret
ota-sign apply --channel ./channel --install ./app --public ./keys/release.public
# або з GitHub raw / Pages:
ota-sign apply-url --base https://raw.githubusercontent.com/You/my-ota/main --install ./app --public ./keys/release.public
```

Публічний канал на GitHub — лише `manifest.*` і `blobs/`; секретний ключ у репо **не** класти.

## У коді хоста

```rust
use ota_sign::apply_from_url;
apply_from_url(base, install_dir, public_key_path)?;
```

Разом із [desktop-remote-kit](https://github.com/RiasJ1Dar/desktop-remote-kit): kit каже «є реліз», `ota-sign` підтягує файли.

## Ліцензія

MIT
