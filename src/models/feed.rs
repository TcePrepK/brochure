//! Display helpers for Feed and Article types (rendering symbols and formatting).

use super::{Article, Feed};
use ratatui::prelude::Style;
use ratatui::style::Color;

impl Feed {
    /// Returns `" [N]"` if there are unread articles, empty string otherwise.
    pub fn unread_badge(&self) -> String {
        if self.unread_count > 0 {
            format!(" [{}]", self.unread_count)
        } else {
            String::new()
        }
    }
}

impl Article {
    /// Returns a symbol indicating the article's read/saved state: filled/empty circle/square.
    pub fn get_icon(&self) -> &'static str {
        if self.is_saved {
            if self.is_read { "□ " } else { "■ " }
        } else {
            if self.is_read { "○ " } else { "● " }
        }
    }

    /// Returns the color for the article's state icon given theme colors.
    ///
    /// Takes the three relevant theme colors: `saved_color` (e.g. yellow),
    /// `read_color` (e.g. subtext0), and `unread_color` (e.g. blue).
    pub fn get_icon_style(
        &self,
        saved_color: Color,
        read_color: Color,
        unread_color: Color,
    ) -> Style {
        if self.is_saved {
            Style::default().fg(saved_color)
        } else if self.is_read {
            Style::default().fg(read_color)
        } else {
            Style::default().fg(unread_color)
        }
    }
}
