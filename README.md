<div align="center">

# brochure

**A terminal RSS reader — keyboard-driven, distraction-free.**

[![Crates.io Version](https://img.shields.io/crates/v/brochure?style=flat-square&color=f5c2e7)](https://crates.io/crates/brochure)
[![Crates.io Downloads](https://img.shields.io/crates/d/brochure?style=flat-square&color=cba6f7)](https://crates.io/crates/brochure)
[![License: MIT](https://img.shields.io/badge/license-MIT-89dceb?style=flat-square)](LICENSE)
[![Rust: 1.85+](https://img.shields.io/badge/rust-1.85+-fab387?style=flat-square)](https://www.rust-lang.org)

Built with [Ratatui](https://ratatui.rs) · Catppuccin Mocha theme · Full RSS/Atom support

</div>

---

## Features

- **RSS & Atom** — both feed formats supported out of the box
- **Categories** — organise feeds into collapsible groups
- **Saved articles** — star any article and group saves by source
- **OPML import/export** — bring your existing subscriptions in, or take them out
- **Readability fetch** — pulls full article body when the feed only provides a summary
- **Fetch policy** — choose when brochure refreshes: on start, every hour, every day, or never
- **Feed editor** — rename, move, and delete feeds and categories without leaving the TUI
- **Catppuccin Mocha** — soft purple colour theme throughout

## Installation

```bash
cargo install brochure
```

Then launch with:

```bash
brochure
```

**Requirements:** Rust 1.85 or later. Install via [rustup](https://rustup.rs) if needed.

## Data & configuration

brochure stores all data in the platform data directory — no config files to manage manually.

| Platform | Path                                      |
|----------|-------------------------------------------|
| Linux    | `~/.local/share/brochure/`                |
| macOS    | `~/Library/Application Support/brochure/` |
| Windows  | `%APPDATA%\brochure\`                     |

Files: `feeds.json`, `articles.json`, `categories.json`, `user_data.json`.

OPML exports go to your **Downloads** folder by default.

## Contributing

Bug reports and feature requests are welcome — [open an issue](https://github.com/TcePrepK/brochure/issues/new).

Pull requests are also welcome. Please run `cargo fmt && cargo clippy` before submitting.

## License

MIT — see [LICENSE](LICENSE).
