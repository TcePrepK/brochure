//! Article detail view rendering with markdown content, scrolling, and header.

use crate::{app::App, handlers::article::get_selected_article};
use limner::{Alignment, MarkdownStyle, render_image::Image, render_markdown_with_extra};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Wrap},
};

use super::super::{SPINNER_FRAMES, content_block, render_scrollbar};
use super::{footer::draw_article_footer, utils::now_secs};

/// Build a limner style config from the app theme.
fn md_style(theme: &crate::ui::theme::ColorTheme) -> MarkdownStyle {
    MarkdownStyle {
        paragraph: Style::new().fg(theme.text),
        paragraph_alignment: Alignment::Left,
        heading_1: Style::new().fg(theme.accent).bold(),
        heading_1_alignment: Alignment::Left,
        heading_2: Style::new().fg(theme.accent).bold(),
        heading_2_alignment: Alignment::Left,
        heading_3: Style::new().fg(theme.accent),
        heading_3_alignment: Alignment::Left,
        bold: Style::new().bold(),
        italic: Style::new().italic(),
        strikethrough: Style::new().crossed_out(),
        inline_code: Style::new().fg(theme.teal).bg(Color::Rgb(40, 40, 40)),
        code_block: Style::new().fg(theme.teal),
        code_block_bg: theme.bg_dark,
        code_block_alignment: Alignment::Left,
        link: Style::new().fg(theme.link).underlined(),
        link_prefix: "🔗 ",
        quote: Style::new().fg(theme.muted_text),
        quote_alignment: Alignment::Left,
        quote_indicator: "▍ ",
        image: Style::new()
            .fg(theme.muted_text)
            .add_modifier(Modifier::ITALIC | Modifier::DIM),
        image_prefix: "📷 ",
        list_bullet: "• ",
        ordered_template: "{}. ",
        hr_char: '─',
        hr_style: Style::new().fg(theme.border),
    }
}

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
        let now = now_secs().max(secs);
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
    let detail_title = if feed_refreshing {
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

    let scroll_offset = app.article_scroll.get(&article.link);

    if !is_preview {
        app.content_area_height = content_area.height;
    }

    let style = md_style(&app.theme);

    // First pass at full width to determine if content overflows.
    let mut render_width = content_area.width;
    let mut result =
        render_markdown_with_extra(&article.content, &style, render_width, &article.images);
    let mut lines = result.lines.clone();
    let mut line_count = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .line_count(render_width)
        .max(1);

    if line_count > content_area.height as usize {
        // Needs scrollbar — re-render at narrower width so separators don't overflow.
        render_width = content_area.width.saturating_sub(2);
        result =
            render_markdown_with_extra(&article.content, &style, render_width, &article.images);
        lines = result.lines.clone();
        line_count = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .line_count(render_width)
            .max(1);
    }

    if !is_preview {
        app.content_line_count = line_count;
    }

    let mut lines = result.lines;

    // Inject inline images into the rendered line buffer (preview and full).
    let placements = if let Some(ref picker) = app.picker
        && !app.image_cache.is_empty()
    {
        let font_size = picker.font_size();
        limner::render_image::prepare_inline_images(
            &mut lines,
            &result.images,
            &app.image_cache,
            &mut app.protocol_cache,
            picker,
            &font_size,
            render_width,
            10,
        )
    } else {
        Vec::new()
    };

    // Store render metadata in the full-detail view only.
    if !is_preview {
        app.article_images = result.images;
        app.article_links = result.links;
        app.article_content_area = content_area;
        app.article_scroll_offset = scroll_offset;
    }

    // Render inline images on top of the reserved empty lines.
    // Convert raw line_start → visual line index (accounts for word-wrap).
    let content_top = content_area.y as i32;
    let content_bottom = (content_area.y + content_area.height) as i32;
    for p in &placements {
        let Some(protocol) = app.protocol_cache.get(&p.url) else {
            continue;
        };

        let visual_y = if p.line_start == 0 {
            0
        } else {
            let end = p.line_start.min(lines.len());
            Paragraph::new(lines[..end].to_vec())
                .wrap(Wrap { trim: false })
                .line_count(render_width)
                .max(1) as u16
        };
        let y0 = content_top + visual_y as i32 - scroll_offset as i32;
        let y1 = y0 + p.cell_rows as i32;

        if y0 < content_top || y1 > content_bottom {
            continue;
        }

        f.render_widget(
            Image::new(protocol),
            Rect {
                x: content_area.x,
                y: y0 as u16,
                width: p.cell_cols.min(content_area.width),
                height: p.cell_rows,
            },
        );
    }

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));

    if show_footer {
        draw_article_footer(f, app, bar_area, true);
    }

    let has_scrollbar = line_count > content_area.height as usize;
    let para_render_area = Rect {
        width: render_width,
        ..content_area
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
