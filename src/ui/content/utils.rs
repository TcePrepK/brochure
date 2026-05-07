//! Pure formatting utilities for timestamps, titles, and colors used across content submodules.

use crate::ui::theme::Theme;
use ratatui::style::Color;

/// Formats a Unix timestamp as a human-readable age string (e.g., "42m ago", "2d ago").
pub(super) fn format_age(secs: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(secs);
    let diff = (now - secs).max(0) as u64;
    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
    }
}

/// Truncate `text` to `max` chars, appending `…` if truncated.
pub(super) fn truncate_title(text: &str, max: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max || max == 0 {
        text.to_string()
    } else {
        let end = max.saturating_sub(1);
        chars[..end].iter().collect::<String>() + "…"
    }
}

/// Scroll `text` to fit within `available` chars, using `elapsed` ticks.
/// Pauses 8 ticks (~2s) before scrolling, then advances 1 char/tick, stops at end.
pub(super) fn scroll_title(text: &str, available: usize, elapsed: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len <= available || available == 0 {
        return text.to_string();
    }
    let max_offset = len - available;
    let start = elapsed.saturating_sub(3).min(max_offset);
    chars[start..start + available].iter().collect()
}

/// Returns a color for a timestamp age: green for recent (< 1h), yellow for today, dimmed for older.
pub(super) fn age_color(secs: i64, theme: &Theme) -> Color {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(secs);
    let diff = (now - secs).max(0) as u64;
    if diff < 3600 {
        theme.green
    } else if diff < 86400 {
        theme.yellow
    } else {
        theme.subtext0
    }
}

/// Formats a Unix timestamp as a compact short age string with a leading space (e.g., " now", " 42m", " 2h", " 3d").
pub(super) fn short_age(secs: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(secs);
    let diff = (now - secs).max(0) as u64;
    if diff < 60 {
        " now".to_string()
    } else if diff < 3600 {
        format!(" {}m", diff / 60)
    } else if diff < 86400 {
        format!(" {}h", diff / 3600)
    } else {
        format!(" {}d", diff / 86400)
    }
}
