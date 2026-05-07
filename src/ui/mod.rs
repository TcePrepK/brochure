//! Terminal UI rendering: tree utilities, and top-level draw dispatcher.
//!
//! This module owns all rendering logic, including tree indentation helpers,
//! and the main `draw()` function that dispatches to per-tab renderers.

mod changelog;
mod chrome;
mod content;
mod feed_editor;
mod popups;
mod saved_category_editor;
mod settings;
pub(crate) mod theme;
mod theme_editor;

use crate::app::App;
use crate::models::{AppState, FeedTreeItem, Tab};
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Stylize};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    symbols,
};

/// Braille spinner animation frames for loading indicators.
pub(crate) const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Returns the border set based on the user's rounded-border preference.
pub(crate) fn border_set(rounded: bool) -> symbols::border::Set<'static> {
    if rounded {
        symbols::border::ROUNDED
    } else {
        symbols::border::PLAIN
    }
}

/// Renders a list with an optional vertical scrollbar.
///
/// When item count exceeds `inner.height`, reserves 1 column on the right for a scrollbar.
/// The scrollbar offset is derived from `list_state.offset()` after rendering, so it always
/// reflects the first visible item rather than the cursor position.
pub(crate) fn render_scrollable_list<'a>(
    f: &mut Frame,
    items: Vec<ListItem<'a>>,
    inner: Rect,
    list_state: &mut ListState,
    theme: &Theme,
) {
    let total = items.len();
    let has_scrollbar = total > inner.height as usize;
    let list_area = if has_scrollbar {
        Rect {
            width: inner.width.saturating_sub(1),
            ..inner
        }
    } else {
        inner
    };
    f.render_stateful_widget(List::new(items), list_area, list_state);
    if has_scrollbar {
        let bar_area = Rect {
            x: inner.right().saturating_sub(1),
            y: inner.y,
            width: 1,
            height: inner.height,
        };
        render_scrollbar(
            f,
            bar_area,
            total,
            inner.height as usize,
            list_state.offset(),
            theme,
        );
    }
}

/// Compute the leading indent string for a tree item at `depth` positioned at `render_idx`.
/// For each ancestor level (1 to depth-1), emits "│  " if that level still has siblings
/// after the current item, or "   " if it was the last child.
pub(crate) fn tree_indent(tree: &[FeedTreeItem], render_idx: usize, depth: u8) -> String {
    if depth <= 1 {
        return String::new();
    }
    let mut s = String::new();
    for level in 1..depth {
        let next_at_level = tree[render_idx + 1..]
            .iter()
            .find(|n| {
                let d = match n {
                    FeedTreeItem::AllFeeds => 0,
                    FeedTreeItem::Feed { depth, .. } | FeedTreeItem::Category { depth, .. } => {
                        *depth
                    }
                };
                d <= level
            })
            .map(|n| match n {
                FeedTreeItem::AllFeeds => 0,
                FeedTreeItem::Feed { depth, .. } | FeedTreeItem::Category { depth, .. } => *depth,
            });
        if next_at_level == Some(level) {
            s.push_str("│  ");
        } else {
            s.push_str("   ");
        }
    }
    s
}

/// Compute the tree connector prefix (`├─ `, `╰─ `/`└─ `, or `root_str`) for an item.
/// `root_str` is returned at depth 0 (e.g., `""` for categories, `"   "` for feeds).
pub(crate) fn tree_connector(
    tree: &[FeedTreeItem],
    idx: usize,
    depth: u8,
    rounded: bool,
    root_str: &'static str,
) -> &'static str {
    if depth == 0 {
        return root_str;
    }
    let is_last = tree[idx + 1..]
        .iter()
        .find(|n| {
            let d = match n {
                FeedTreeItem::AllFeeds => 0,
                FeedTreeItem::Feed { depth, .. } | FeedTreeItem::Category { depth, .. } => *depth,
            };
            d <= depth
        })
        .is_none_or(|n| {
            let d = match n {
                FeedTreeItem::AllFeeds => 0,
                FeedTreeItem::Feed { depth, .. } | FeedTreeItem::Category { depth, .. } => *depth,
            };
            d < depth
        });
    if is_last {
        if rounded { "╰─ " } else { "└─ " }
    } else {
        "├─ "
    }
}

/// Build a titled border block with the project's standard style.
///
/// `title` is shown in the top-left corner; `focused` controls border color (mauve vs surface0).
/// `rounded` controls border symbols (rounded vs plain).
pub(crate) fn content_block<'a, T>(
    title: T,
    focused: bool,
    rounded: bool,
    theme: &Theme,
) -> Block<'a>
where
    T: Into<Line<'a>>,
{
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_set(border_set(rounded))
        .border_style(Style::default().fg(if focused { theme.accent } else { theme.border }))
        .bg(theme.bg)
}

/// Renders a themed vertical scrollbar into `area`.
///
/// Applies the standard arrow, thumb, and track styling from the active theme.
/// Call this whenever `content_len > viewport_len` to show a scrollbar.
pub(crate) fn render_scrollbar(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    content_len: usize,
    viewport_len: usize,
    offset: usize,
    theme: &Theme,
) {
    use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
    f.render_widget(
        &ScrollBar::vertical(ScrollLengths {
            content_len,
            viewport_len,
        })
        .arrows(ScrollBarArrows::Both)
        .arrow_style(Style::default().fg(theme.bg_dark).bg(theme.bg))
        .thumb_style(Style::default().fg(theme.border).bg(theme.bg_dark))
        .track_style(Style::default().bg(theme.bg_dark))
        .offset(offset),
        area,
    );
}

/// Top-level draw dispatcher that renders the entire UI frame.
///
/// Dispatches to per-tab renderers (Feeds, Saved, Settings) and overlays state-specific popups
/// (add-feed wizard, OPML paths, confirm dialogs, category picker, saved category editor).
pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    chrome::draw_tab_bar(f, app, chunks[0]);
    chrome::draw_footer(f, app, chunks[2]);

    if matches!(
        app.state,
        AppState::SavedCategoryEditor
            | AppState::SavedCategoryEditorRename
            | AppState::SavedCategoryEditorDeleteConfirm
            | AppState::SavedCategoryEditorNew
    ) {
        saved_category_editor::draw_saved_category_editor(f, app, chunks[1]);
        if app.state == AppState::SavedCategoryEditorDeleteConfirm {
            popups::draw_confirm_delete_saved_cat(f, app);
        }
        return;
    }

    if matches!(
        app.state,
        AppState::ThemeEditor
            | AppState::ThemeEditorNew
            | AppState::ThemeEditorColorEdit
            | AppState::ThemeEditorHexInput
            | AppState::ThemeEditorRename
            | AppState::ThemeEditorExport
            | AppState::ThemeEditorImport
    ) {
        theme_editor::draw_theme_editor_screen(f, app, chunks[1]);
        return;
    }

    match app.selected_tab {
        Tab::Feeds => content::draw_feeds_tab(f, app, chunks[1]),
        Tab::Saved => content::draw_saved_tab(f, app, chunks[1]),
        Tab::Settings => settings::draw_settings_tab(f, app, chunks[1]),
        Tab::Changelog => changelog::draw_changelog_tab(f, app, chunks[1]),
    }

    if app.state == AppState::AddFeed {
        popups::draw_add_feed_popup(f, app);
    }
    if matches!(
        app.state,
        AppState::OPMLExportPath | AppState::OPMLImportPath
    ) {
        popups::draw_opml_path_popup(f, app);
    }
    if app.state == AppState::ClearData {
        popups::draw_confirm_delete_all(f, app);
    }
    if app.state == AppState::ClearArticleCache {
        popups::draw_confirm_clear_cache(f, app);
    }
    if let Some((cat_id, feed_count)) = app.editor_delete_cat {
        popups::draw_confirm_delete_cat(f, app, cat_id, feed_count);
    }
    if app.state == AppState::CategoryPicker {
        popups::draw_category_picker(f, app);
    }
    if app.update_available.is_some() {
        popups::draw_update_popup(f, app);
    }
}
