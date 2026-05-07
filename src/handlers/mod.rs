//! Key event handlers routed by app state.
//!
//! Dispatches keyboard input to state-specific handlers (feed list, article detail, settings, etc.).

pub(crate) mod article;
mod changelog;
mod feed_editor;
mod feed_list;
mod saved_category_editor;
mod settings;
mod theme_editor;

use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    app::App,
    models::{AppEvent, AppState},
};

/// Cursor-aware text input: handles Left/Right movement, Backspace (delete before cursor),
/// and Char insertion at cursor. Shared by all text-input handler modules.
pub(super) fn handle_text_input(input: &mut String, cursor: &mut usize, key: KeyCode) {
    match key {
        KeyCode::Left if *cursor > 0 => {
            *cursor -= 1;
        }
        KeyCode::Right if *cursor < input.chars().count() => {
            *cursor += 1;
        }
        KeyCode::Backspace if *cursor > 0 => {
            let byte_idx = input
                .char_indices()
                .nth(*cursor - 1)
                .map(|(i, _)| i)
                .unwrap_or(input.len());
            input.remove(byte_idx);
            *cursor -= 1;
        }
        KeyCode::Char(c) => {
            let byte_idx = input
                .char_indices()
                .nth(*cursor)
                .map(|(i, _)| i)
                .unwrap_or(input.len());
            input.insert(byte_idx, c);
            *cursor += 1;
        }
        _ => {}
    }
}

/// Route a key event to the correct handler based on the current app state.
pub async fn handle_key(app: &mut App, key: KeyEvent, tx: &UnboundedSender<AppEvent>) -> bool {
    if app.update_available.is_some() {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.update_popup_scroll = app.update_popup_scroll.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.update_popup_scroll = app.update_popup_scroll.saturating_add(1);
            }
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                app.update_available = None;
                app.update_popup_scroll = 0;
            }
            _ => {}
        }
        return false;
    }
    match app.state {
        AppState::AddFeed => settings::handle_add_feed(app, key, tx),
        AppState::SettingsList => return settings::handle_settings(app, key),
        AppState::OPMLExportPath | AppState::OPMLImportPath => {
            settings::handle_opml_path(app, key, tx)
        }
        AppState::ClearData => settings::handle_confirm_delete_all(app, key),
        AppState::ClearArticleCache => settings::handle_confirm_clear_cache(app, key),
        AppState::ArticleList | AppState::ArticleDetail => {
            return article::handle_article(app, key, tx).await;
        }
        AppState::FeedList => {
            let should_quit = feed_list::handle_feed_list(app, key, tx);
            if app.state == AppState::ArticleList {
                article::prefetch_article_if_stub(app, tx);
            }
            return should_quit;
        }
        AppState::SavedCategoryList => return feed_list::handle_saved_feed_list(app, key),
        AppState::FeedEditor | AppState::FeedEditorRename => {
            feed_editor::handle_feed_editor(app, key, tx)
        }
        AppState::CategoryPicker => article::handle_category_picker(app, key, tx),
        AppState::SavedCategoryEditor => {
            saved_category_editor::handle_saved_category_editor(app, key)
        }
        AppState::SavedCategoryEditorRename => {
            saved_category_editor::handle_saved_category_editor_rename(app, key)
        }
        AppState::SavedCategoryEditorDeleteConfirm => {
            saved_category_editor::handle_saved_category_editor_delete_confirm(app, key)
        }
        AppState::SavedCategoryEditorNew => {
            saved_category_editor::handle_saved_category_editor_new(app, key)
        }
        AppState::Changelog => return changelog::handle_changelog(app, key),
        AppState::ThemeEditor => theme_editor::handle_theme_editor(app, key),
        AppState::ThemeEditorNew => theme_editor::handle_theme_editor_new(app, key),
        AppState::ThemeEditorColorEdit => theme_editor::handle_theme_editor_color_edit(app, key),
        AppState::ThemeEditorHexInput => theme_editor::handle_theme_editor_hex_input(app, key),
        AppState::ThemeEditorRename => theme_editor::handle_theme_editor_rename(app, key),
        AppState::ThemeEditorExport => theme_editor::handle_theme_editor_export(app, key),
        AppState::ThemeEditorImport => theme_editor::handle_theme_editor_import(app, key),
    }
    false
}
