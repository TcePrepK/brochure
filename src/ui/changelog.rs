//! Rendering for the Changelog/About tab.
//!
//! Displays a static About block (app name, version, description) above a
//! scrollable list of changelog entries parsed from the embedded `changelog.json`.

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::{content_block, render_scrollbar};

const CHANGELOG_JSON: &str = include_str!("../../changelog.json");

#[derive(serde::Deserialize)]
struct ChangelogEntry {
    version: String,
    date: String,
    summary: String,
    highlights: Vec<String>,
}

/// Renders the Changelog tab: an About block at the top and a scrollable changelog list below.
pub(super) fn draw_changelog_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .split(area);

    draw_about_block(f, app, chunks[0]);
    draw_changelog_block(f, app, chunks[1]);
}

/// Renders the About block showing the app name, version, and a short description.
fn draw_about_block(f: &mut Frame, app: &App, area: Rect) {
    let version = env!("CARGO_PKG_VERSION");

    let block = content_block(
        " About ".fg(app.theme.notice).bold(),
        false,
        app.user_data.border_rounded,
        &app.theme,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let name_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(
            "Brochure",
            Style::default()
                .fg(app.theme.text)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("v{version}"),
            Style::default()
                .fg(app.theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let desc_line = Line::from(vec![Span::styled(
        "  A terminal RSS reader built with Ratatui",
        Style::default().fg(app.theme.muted_text),
    )]);

    let blank_line = Line::raw("");
    let author_line = Line::from(vec![
        Span::styled("  Author:      ", Style::default().fg(app.theme.muted_text)),
        Span::styled("Sylviromi", Style::default().fg(app.theme.text)),
    ]);
    let license_line = Line::from(vec![
        Span::styled("  License:     ", Style::default().fg(app.theme.muted_text)),
        Span::styled("MIT", Style::default().fg(app.theme.text)),
    ]);
    let repo_line = Line::from(vec![
        Span::styled("  Repository:  ", Style::default().fg(app.theme.muted_text)),
        Span::styled(
            "https://github.com/Sylviromi/brochure",
            Style::default().fg(app.theme.link),
        ),
    ]);

    f.render_widget(
        Paragraph::new(vec![
            name_line,
            desc_line,
            blank_line,
            author_line,
            license_line,
            repo_line,
        ])
        .bg(app.theme.bg),
        inner,
    );
}

/// Renders the scrollable changelog entries block.
fn draw_changelog_block(f: &mut Frame, app: &mut App, area: Rect) {
    let block = content_block(
        " Changelog ".fg(app.theme.notice).bold(),
        true,
        app.user_data.border_rounded,
        &app.theme,
    );
    let inner = block.inner(area);
    f.render_widget(block, area);

    let entries: Vec<ChangelogEntry> = match serde_json::from_str(CHANGELOG_JSON) {
        Ok(v) => v,
        Err(e) => {
            f.render_widget(
                Paragraph::new(format!("Error parsing changelog.json: {e}"))
                    .style(Style::default().fg(app.theme.text))
                    .bg(app.theme.bg),
                inner,
            );
            return;
        }
    };

    let mut lines: Vec<Line> = Vec::new();
    let total = entries.len();
    for (i, entry) in entries.iter().rev().enumerate() {
        // Determine the tree connector based on position
        let connector = if i == total - 1 {
            if app.user_data.border_rounded {
                " ╰─"
            } else {
                " └─"
            }
        } else {
            " ├─"
        };
        let prefix = if i == total - 1 { "    " } else { " │  " };

        // Version line with connector
        lines.push(Line::from(vec![
            Span::styled(
                format!("{connector} "),
                Style::default().fg(app.theme.border),
            ),
            Span::styled(
                format!("v{}", entry.version),
                Style::default()
                    .fg(app.theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ·  ", Style::default().fg(app.theme.border)),
            Span::styled(entry.date.clone(), Style::default().fg(app.theme.muted_text)),
        ]));
        // Summary line with vertical bar
        lines.push(Line::from(vec![
            Span::styled(prefix, Style::default().fg(app.theme.border)),
            Span::styled(entry.summary.clone(), Style::default().fg(app.theme.text)),
        ]));
        // Highlights with vertical bar
        for highlight in &entry.highlights {
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(app.theme.border)),
                Span::styled(
                    format!("  • {highlight}"),
                    Style::default().fg(app.theme.muted_text),
                ),
            ]));
        }
        // Separator line (vertical bar or nothing after last entry)
        if i + 1 < total {
            lines.push(Line::from(vec![Span::styled(
                prefix,
                Style::default().fg(app.theme.border),
            )]));
        }
    }

    let viewport_height = inner.height as usize;
    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(viewport_height) as u16;
    if app.changelog_scroll > max_scroll {
        app.changelog_scroll = max_scroll;
    }

    let has_scrollbar = total_lines > viewport_height;
    let text_area = if has_scrollbar {
        Rect {
            width: inner.width.saturating_sub(1),
            ..inner
        }
    } else {
        inner
    };

    let visible: Vec<Line> = lines
        .into_iter()
        .skip(app.changelog_scroll as usize)
        .take(viewport_height)
        .collect();

    f.render_widget(Paragraph::new(visible).bg(app.theme.bg), text_area);

    if has_scrollbar {
        let bar_area = Rect {
            x: inner.x + inner.width.saturating_sub(1),
            width: 1,
            ..inner
        };
        render_scrollbar(
            f,
            bar_area,
            total_lines,
            viewport_height,
            app.changelog_scroll as usize,
            &app.theme,
        );
    }
}
