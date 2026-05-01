//! Display helpers for Feed and Article types (rendering symbols and formatting).

use super::{Article, Feed};
use crate::ui::{BLUE, SUBTEXT0, YELLOW};
use ratatui::prelude::Style;

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

    /// Returns the color for the article's state icon.
    pub fn get_icon_style(&self) -> Style {
        if self.is_saved {
            Style::default().fg(YELLOW)
        } else if self.is_read {
            Style::default().fg(SUBTEXT0)
        } else {
            Style::default().fg(BLUE)
        }
    }
}
