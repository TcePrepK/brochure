//! Settings menu and saved-category editor rendering.
//!
//! This module renders the Settings tab with toggles and buttons for user preferences,
//! and the full-screen saved-category editor for managing custom article categories.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use ratatui::prelude::Stylize;

use crate::{
    app::App,
    models::{AppState, SettingsItem},
};

use super::border_set;

/// Dispatches to the settings list renderer when in a settings-related state.
pub(super) fn draw_settings_tab(f: &mut Frame, app: &App, area: Rect) {
    match app.state {
        AppState::SettingsList
        | AppState::OPMLExportPath
        | AppState::OPMLImportPath
        | AppState::ClearData
        | AppState::ClearArticleCache => draw_settings(f, app, area),
        _ => {}
    }
}

/// Formats a byte count as a human-readable size string (B, KB, MB).
fn format_cache_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes == 0 {
        "empty".to_string()
    } else {
        format!("{bytes} B")
    }
}

/// Renders the settings menu with grouped sections, toggles, and action buttons.
fn draw_settings(f: &mut Frame, app: &App, area: Rect) {
    enum Row {
        SectionHeader {
            label: &'static str,
            is_last: bool,
        },
        Item {
            item: SettingsItem,
            label: &'static str,
            in_last: bool,
        },
        Toggle {
            item: SettingsItem,
            label: &'static str,
            in_last: bool,
            on: bool,
        },
        Cycle {
            item: SettingsItem,
            label: &'static str,
            in_last: bool,
            value: String,
        },
        CacheItem {
            in_last: bool,
            size_label: String,
        },
        Spacer,
    }

    let save = app.user_data.save_article_content;
    let eager = app.user_data.eager_article_fetch;
    let rounded = app.user_data.border_rounded;
    let scroll_loop = app.user_data.scroll_loop;
    let cache_label = format_cache_size(app.article_cache_size);
    let rows = [
        Row::SectionHeader {
            label: " Data",
            is_last: false,
        },
        Row::Item {
            item: SettingsItem::ImportOpml,
            label: "[ Import OPML ]",
            in_last: false,
        },
        Row::Item {
            item: SettingsItem::ExportOpml,
            label: "[ Export OPML ]",
            in_last: false,
        },
        Row::Item {
            item: SettingsItem::ClearData,
            label: "[ Clear All Data ]",
            in_last: false,
        },
        Row::Spacer,
        Row::SectionHeader {
            label: " Article Storage",
            is_last: false,
        },
        Row::Toggle {
            item: SettingsItem::SaveArticleContent,
            label: "[ Save Article Content ]",
            in_last: false,
            on: save,
        },
        Row::CacheItem {
            in_last: false,
            size_label: cache_label,
        },
        Row::Spacer,
        Row::SectionHeader {
            label: " Fetching",
            is_last: false,
        },
        Row::Toggle {
            item: SettingsItem::EagerArticleFetch,
            label: "[ Eager Article Fetch ]",
            in_last: false,
            on: eager,
        },
        Row::Cycle {
            item: SettingsItem::AutoFetchOnStart,
            label: "[ Fetch Policy ]",
            in_last: false,
            value: app.user_data.fetch_policy.label().to_string(),
        },
        Row::Cycle {
            item: SettingsItem::ArchivePolicy,
            label: "[ Archive Policy ]",
            in_last: false,
            value: app.user_data.archive_policy.label().to_string(),
        },
        Row::Spacer,
        Row::SectionHeader {
            label: " Appearance",
            is_last: true,
        },
        Row::Toggle {
            item: SettingsItem::ScrollLoop,
            label: "[ Scroll Loop ]",
            in_last: true,
            on: scroll_loop,
        },
        Row::Toggle {
            item: SettingsItem::BorderStyle,
            label: "[ Rounded Borders ]",
            in_last: true,
            on: rounded,
        },
        Row::Cycle {
            item: SettingsItem::Theme,
            label: "[ Theme ]",
            in_last: true,
            value: app.theme.name.clone(),
        },
    ];

    let list_items: Vec<ListItem> = rows
        .iter()
        .map(|row| match row {
            Row::SectionHeader { label, is_last } => {
                let connector = if *is_last {
                    if app.user_data.border_rounded {
                        " ╰─"
                    } else {
                        " └─"
                    }
                } else {
                    " ├─"
                };
                ListItem::new(Line::from(vec![
                    connector.fg(app.theme.surface0),
                    label.fg(app.theme.peach).bold(),
                ]))
            }
            Row::Item {
                item,
                label,
                in_last,
            } => {
                let prefix = if *in_last { "     " } else { " │   " };
                let selected = app.settings_selected == *item;
                let style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                ListItem::new(Line::from(vec![
                    prefix.fg(app.theme.surface0),
                    Span::styled(*label, style),
                ]))
            }
            Row::Toggle {
                item,
                label,
                in_last,
                on,
            } => {
                let prefix = if *in_last { "     " } else { " │   " };
                let selected = app.settings_selected == *item;
                let base_style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                let badge_style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else if *on {
                    Style::default()
                        .fg(app.theme.green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.subtext0)
                };
                ListItem::new(Line::from(vec![
                    prefix.fg(app.theme.surface0),
                    Span::styled(*label, base_style),
                    Span::styled(if *on { "  ON " } else { "  OFF " }, badge_style),
                ]))
            }
            Row::Cycle {
                item,
                label,
                in_last,
                value,
            } => {
                let prefix = if *in_last { "     " } else { " │   " };
                let selected = app.settings_selected == *item;
                let base_style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                let badge_style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(app.theme.yellow)
                        .add_modifier(Modifier::BOLD)
                };
                ListItem::new(Line::from(vec![
                    prefix.fg(app.theme.surface0),
                    Span::styled(*label, base_style),
                    Span::styled(format!("  {}", value), badge_style),
                ]))
            }
            Row::CacheItem {
                in_last,
                size_label,
            } => {
                let prefix = if *in_last { "     " } else { " │   " };
                let selected = app.settings_selected == SettingsItem::ClearArticleCache;
                let base_style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.text)
                };
                let badge_style = if selected {
                    Style::default()
                        .fg(app.theme.mantle)
                        .bg(app.theme.mauve)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.red)
                };
                ListItem::new(Line::from(vec![
                    prefix.fg(app.theme.surface0),
                    Span::styled("[ Clear Article Cache ]", base_style),
                    Span::styled(format!("  {} ", size_label), badge_style),
                ]))
            }
            Row::Spacer => ListItem::new(Line::from(" │".fg(app.theme.surface0))),
        })
        .collect();

    let block = Block::default()
        .border_set(border_set(app.user_data.border_rounded))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.state == AppState::SettingsList {
            app.theme.mauve
        } else {
            app.theme.surface0
        }))
        .bg(app.theme.base)
        .title(" Settings ".fg(app.theme.peach).bold());

    f.render_widget(List::new(list_items).block(block), area);
}

/// Renders the full-screen saved-category editor for renaming and deleting custom categories.
pub(super) fn draw_saved_category_editor(f: &mut Frame, app: &mut App, area: Rect) {
    let rounded = app.user_data.border_rounded;
    let block = Block::default()
        .title(" Saved Category Editor  [r] rename  [d] delete  [n] new  [Esc] back ")
        .borders(Borders::ALL)
        .border_set(border_set(rounded))
        .border_style(Style::default().fg(app.theme.mauve))
        .style(Style::default().bg(app.theme.base));

    f.render_widget(block.clone(), area);
    let inner = block.inner(area);

    if app.user_data.saved_categories.is_empty() {
        let msg = Paragraph::new("  No categories yet. Save an article with [s] to create one.")
            .style(Style::default().fg(app.theme.subtext0));
        f.render_widget(msg, inner);
        return;
    }

    let items: Vec<ListItem> = app
        .user_data
        .saved_categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let count = app
                .user_data
                .saved_articles
                .iter()
                .filter(|s| s.category_id == cat.id)
                .count();

            let name_line = if app.state == AppState::SavedCategoryEditorRename
                && i == app.saved_cat_editor_scroll.cursor
            {
                let chars: Vec<char> = app.editor_input.chars().collect();
                let pos = app.input_cursor.min(chars.len());
                let before: String = chars[..pos].iter().collect();
                let after: String = chars[pos..].iter().collect();
                Line::from(vec![
                    "  ".fg(app.theme.yellow),
                    before.fg(app.theme.yellow),
                    "|".fg(app.theme.mauve).bold(),
                    after.fg(app.theme.yellow),
                ])
            } else {
                let style = if i == app.saved_cat_editor_scroll.cursor {
                    Style::default().bg(app.theme.surface0).fg(app.theme.mauve)
                } else {
                    Style::default().fg(app.theme.text)
                };
                Line::from(Span::styled(format!("  {}", cat.name), style))
            };

            let count_span = Span::styled(
                format!("  [{count} article{}]", if count == 1 { "" } else { "s" }),
                Style::default().fg(app.theme.subtext0),
            );

            let mut spans = name_line.spans;
            spans.push(count_span);
            ListItem::new(Line::from(spans))
        })
        .collect();

    render_scrollable_list!(
        f,
        items,
        inner,
        app.saved_cat_editor_scroll.list_state,
        app.saved_cat_editor_scroll.cursor,
        &app.theme
    );

    // Render input row when creating a new category.
    if app.state == AppState::SavedCategoryEditorNew {
        let input_area = Rect {
            y: area.bottom().saturating_sub(2),
            height: 1,
            ..area
        };
        let chars: Vec<char> = app.editor_input.chars().collect();
        let pos = app.input_cursor.min(chars.len());
        let before: String = chars[..pos].iter().collect();
        let after: String = chars[pos..].iter().collect();
        let input_line = Line::from(vec![
            "  Category name: ".fg(app.theme.yellow),
            before.fg(app.theme.yellow),
            "|".fg(app.theme.mauve).bold(),
            after.fg(app.theme.yellow),
        ]);
        let input_para = Paragraph::new(input_line);
        f.render_widget(input_para, input_area);
    }
}
