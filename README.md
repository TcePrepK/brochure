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
- **Themes** — five built-in colour themes (Catppuccin Mocha, Gruvbox Dark, Dracula, Nord, GNOME) plus custom TOML themes

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

## Themes

brochure ships five built-in themes: **Catppuccin Mocha**, **Gruvbox Dark**, **Dracula**, **Nord**, and **GNOME**.

Open the theme editor from **Settings → Theme**.

### Theme editor

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate theme list |
| `Enter` | Activate selected theme |
| `n` | New custom theme (clone from any existing theme) |
| `e` | Edit colors (custom themes only) |
| `r` | Rename (custom themes only) |
| `d` | Delete (custom themes only) |
| `x` | Export theme to a `.toml` file |
| `i` | Import a `.toml` file as a new custom theme |

When editing colors, navigate with `↑/↓`, press `Enter` on a slot to type a new `#rrggbb` hex value. A live color swatch previews your input. Press `s` or `Esc` to return.

### Custom theme TOML format

```toml
name = "My Theme"

[colors]
mauve    = "#cba6f7"   # accent / focused border
blue     = "#89b4fa"   # links / highlights
green    = "#a6e3a1"   # success / read indicator
peach    = "#fab387"   # section headers / warnings
base     = "#1e1e2e"   # main background
mantle   = "#181825"   # darkest background (tab bar, footer)
text     = "#cdd6f4"   # primary foreground
subtext0 = "#a6adc8"   # secondary / muted text
surface0 = "#313244"   # unfocused borders
yellow   = "#f9e2af"   # warnings / stars / unread accent
teal     = "#94e2d5"   # teal accent
sky      = "#89dceb"   # lighter blue accent
pink     = "#f5c2e7"   # pink accent
red      = "#f38ba8"   # errors / delete actions
```

All 14 keys are required. Custom themes are stored inline in `user_data.json` — no external file needed after import. You can have any number of custom themes.

## Contributing

Bug reports and feature requests are welcome — [open an issue](https://github.com/TcePrepK/brochure/issues/new).

Pull requests are also welcome. Please run `cargo fmt && cargo clippy` before submitting.

## License

MIT — see [LICENSE](LICENSE).
