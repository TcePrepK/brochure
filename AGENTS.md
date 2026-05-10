# Brochure — Agent Guide

Single binary crate (`src/main.rs` entrypoint, tokio runtime). No workspace, no lib crate.

## Architecture

- **`app.rs`** — central `App` struct holding all state. Methods for navigation, tree traversal, cursor movement.
- **`state/`** — extracted sub-structs, each owning its own text input and cursor:
  `FeedEditorState`, `AddFeedState`, `CategoryPickerState`, `OpmlState`.
  `ThemeEditorState` lives in `models/theme/editor.rs`.
- **`handlers/`** — one file per feature (`article.rs`, `feed_editor.rs`, `settings.rs`, etc.),
  routed by `AppState` in `handlers/mod.rs::handle_key`.
- **`ui/`** — one file per screen, mirrors handler structure.
- **`models/`** — domain types (`Feed`, `Article`, `AppState`, `FeedTreeItem` in `tree.rs`, etc.).
- **`storage.rs`** — disk persistence (JSON files in platform data dir).
- **`fetch.rs`** — async feed fetching via reqwest.

Key pattern: handlers mutate `App`, UI reads `App`. `App::unselect()` handles back-navigation for each state.

## Verification (run in order)

```sh
cargo fmt && cargo fix --allow-dirty && cargo check && cargo clippy && cargo test
```

All 13 tests are inline unit tests — no integration tests, no external services.

## Doc convention

Every `pub` item gets `///` doc comments (one sentence minimum). These feed the AST cache tool.

## AST cache (via `synopsis`)

```sh
synopsis get src/app.rs --compact   # list all items with sigs + docs
synopsis get src/app.rs::App        # source of one item (JSON)
synopsis get src/app.rs::fn::handle_key  # filtered by kind + name
synopsis index src/                 # re-index after edits
```

## Text input helper

All cursor-aware text input goes through `handlers/mod.rs::handle_text_input(input, cursor, key, max_len)`.
`max_len: Some(6)` for hex color codes; `None` for unlimited inputs.
UI rendering uses `ui/content/mod.rs::split_cursor` for cursor display.
