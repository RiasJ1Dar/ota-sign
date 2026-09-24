**Українська** · [English](README.en.md)

# ota-sign

Відкритий інкрементний OTA-формат: Ed25519-підписаний маніфест і
content-addressed блоби за SHA-512. Репозиторій містить CLI для автора релізу та
Rust-бібліотеку для вбудовування в застосунок.

Ідея походить від `szi-as-ota`: передавати лише змінені файли. Формат 2
навмисно прозорий. Конфіденційності й обфускації немає; автентичність дає
підпис, а цілісність кожного файла перевіряється за SHA-512.

## Вимоги та встановлення

Потрібен Rust toolchain із Cargo.

```bash
cargo install --git https://github.com/RiasJ1Dar/ota-sign
```

Або з клону:

```bash
git clone https://github.com/RiasJ1Dar/ota-sign.git
cd ota-sign
cargo install --path .
```

## Перший канал оновлень

Згенеруй пару ключів один раз:

```bash
ota-sign keygen ./keys/release
```

Команда створить `release.secret` і `release.public`. Секретний ключ
залишається лише в середовищі публікації; `*.secret` не можна додавати в Git.
Публічний ключ постачається разом із клієнтом.

Опублікуй вміст `dist/`:

```bash
ota-sign publish \
  --source ./dist \
  --out ./channel \
  --app MyApp \
  --version 1.2.0 \
  --secret ./keys/release.secret
```

Результат:

```text
channel/
├── manifest.json
├── manifest.sig
└── blobs/
    └── <sha512>.bin
```

Файли з однаковим вмістом посилаються на один blob. Приховані файли й теки,
ім'я яких починається з крапки, під час публікації пропускаються.

## Застосування оновлення

Локальний канал:

```bash
ota-sign apply \
  --channel ./channel \
  --install ./app \
  --public ./keys/release.public
```

Статичний HTTP, GitHub Raw або Pages:

```bash
ota-sign apply-url \
  --base https://raw.githubusercontent.com/You/my-ota/main \
  --install ./app \
  --public ./keys/release.public
```

Клієнт спочатку перевіряє підпис маніфесту. Для кожного запису він пропускає
локальний файл із правильним SHA-512, інакше отримує blob, перевіряє його хеш і
замінює файл через тимчасовий `.ota-tmp`.

Файли, яких немає в маніфесті, не видаляються. Це формат доставки й
матеріалізації, а не повна синхронізація теки.

## Команди

| Команда | Призначення |
|---|---|
| `keygen PREFIX` | Створити `PREFIX.secret` і `PREFIX.public` у hex |
| `publish --source DIR --out DIR --app ID --version VER --secret PATH` | Побудувати й підписати канал |
| `apply --channel DIR --install DIR --public PATH` | Перевірити й застосувати локальний канал |
| `apply-url --base URL --install DIR --public PATH` | Завантажити й застосувати HTTP(S)-канал |

`apply` та `apply-url` друкують версію, кількість оновлених і пропущених файлів.

## У коді хоста

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

Публічний API також містить `apply_from_dir`, `publish_dir`,
`generate_keypair`, `Manifest`, `FileEntry` і функції завантаження ключів.

## Формат і гарантії

Повний опис: [docs/format.md](docs/format.md).

- `manifest.sig` — Ed25519-підпис сирих байтів `manifest.json`;
- шляхи в маніфесті відносні, з роздільником `/`, без порожніх компонентів,
  `.` і `..`;
- назва blob — lowercase SHA-512 його відкритого вмісту;
- перед записом клієнт перевіряє і підпис, і хеш;
- блоби не шифруються;
- формат каналу не виконує Windows Authenticode-підписування бінарників.

Разом із [desktop-remote-kit](https://github.com/RiasJ1Dar/desktop-remote-kit):
kit перевіряє, чи є новий GitHub Release, а `ota-sign` доставляє його файли.
[kit-packager](https://github.com/RiasJ1Dar/kit-packager) автоматизує build,
publish і GitHub Release.

## Розробка

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## Ліцензія

MIT