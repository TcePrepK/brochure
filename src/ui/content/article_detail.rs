//! Article detail view rendering with markdown content, scrolling, and header.

use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Paragraph, Wrap},
};

use super::super::{SPINNER_FRAMES, content_block, render_scrollbar};
use super::footer::draw_article_footer;
use crate::app::App;
use crate::handlers::article::get_selected_article;

/// Renders the article detail view with markdown content, scrolling, and article metadata footer.
///
/// When `show_footer` is `false` (called from three-panel mode), the per-panel footer is
/// suppressed; `draw_three_panel` renders a single shared footer instead.
pub(super) fn draw_article_detail(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    is_preview: bool,
    show_footer: bool,
) {
    let article = get_selected_article(app);
    if article.is_none() && is_preview {
        let block = content_block("", false, app.user_data.border_rounded, &app.theme);
        let inner = block.inner(area);
        f.render_widget(block, area);
        f.render_widget(
            Paragraph::new("Select an article to preview.")
                .style(Style::default().fg(app.theme.muted_text)),
            inner,
        );
        return;
    }
    let Some(article) = article else { return };

    // Show spinner only when the article's own feed is actively refreshing.
    let feed_refreshing = !app.in_saved_context
        && !app.in_category_context
        && app.feeds.get(app.selected_feed).is_some_and(|f| !f.fetched);
    let age_suffix: String = if let Some(secs) = article.published_secs {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(secs);
        let diff = (now - secs).max(0) as u64;
        let age = if diff < 60 {
            "now".to_string()
        } else if diff < 3600 {
            format!("{}m", diff / 60)
        } else if diff < 86400 {
            format!("{}h", diff / 3600)
        } else {
            format!("{}d", diff / 86400)
        };
        format!("  • {age}")
    } else {
        String::new()
    };
    let detail_title = if feed_refreshing || app.article_fetching {
        let spinner = SPINNER_FRAMES[app.tick % SPINNER_FRAMES.len()];
        format!(" {spinner} {}{age_suffix} ", article.title)
    } else {
        format!(" {}{age_suffix} ", article.title)
    };
    let block = content_block(
        detail_title.fg(app.theme.accent).bold(),
        !is_preview,
        app.user_data.border_rounded,
        &app.theme,
    );
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let (content_area, bar_area) = if show_footer {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(inner_area);
        (layout[0], layout[1])
    } else {
        (inner_area, Rect::default())
    };

    // Convert images to inline text (![alt](url) → 🖼 alt), then strip hyperlinks.
    let no_images = regex::Regex::new(r"!\[([^]]*)]\([^)]+\)")
        .unwrap()
        .replace_all(&article.content, |caps: &regex::Captures| {
            let alt = caps[1].trim();
            if alt.is_empty() {
                "🖼".to_string()
            } else {
                format!("🖼 {alt}")
            }
        })
        .to_string();
    let stripped = regex::Regex::new(r"\[([^]]+)]\([^)]+\)")
        .unwrap()
        .replace_all(&no_images, "$1")
        .to_string();

    let scroll_offset = app.article_scroll.get(&article.link);

    if !is_preview {
        app.content_area_height = content_area.height;
    }

    // Build the paragraph first so we can call line_count(width) for the true rendered
    // line count (accounts for word-wrap), not just the logical markdown line count.
    let paragraph = Paragraph::new(tui_markdown::from_str(&stripped))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));

    let line_count = paragraph.line_count(content_area.width).max(1);
    if !is_preview {
        app.content_line_count = line_count;
    }

    if show_footer {
        draw_article_footer(f, app, bar_area, true);
    }

    let has_scrollbar = line_count > content_area.height as usize;
    let para_render_area = if has_scrollbar {
        Rect {
            width: content_area.width.saturating_sub(1),
            ..content_area
        }
    } else {
        content_area
    };
    f.render_widget(paragraph, para_render_area);

    if has_scrollbar {
        let bar_area = Rect {
            x: content_area.x + content_area.width.saturating_sub(1),
            width: 1,
            ..content_area
        };
        render_scrollbar(
            f,
            bar_area,
            line_count,
            content_area.height as usize,
            scroll_offset as usize,
            &app.theme,
        );
    }
}
