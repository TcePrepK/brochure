//! The 14 named color slots that make up a custom theme palette.

use serde::{Deserialize, Serialize};

/// The 14 named color slots that make up a theme palette, stored as `#rrggbb` hex strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeColors {
    /// Accent color — used for focused borders and highlighted UI elements.
    pub accent: String,
    /// Link color — hyperlinks and interactive text.
    pub link: String,
    /// Success/positive indicator color.
    pub success: String,
    /// Notice/warning color.
    pub notice: String,
    /// Main background color.
    pub bg: String,
    /// Dark background — panels and sidebars.
    pub bg_dark: String,
    /// Primary text color.
    pub text: String,
    /// Secondary/muted text color.
    pub muted_text: String,
    /// Unfocused border color.
    pub border: String,
    /// Unread indicator color.
    pub unread: String,
    /// Teal accent — used for category colors and decorative elements.
    pub teal: String,
    /// Sky blue accent — used for category colors.
    pub sky: String,
    /// Pink accent — used for category colors and decorative elements.
    pub pink: String,
    /// Error/destructive action color.
    pub error: String,
}

impl CustomThemeColors {
    /// Get a color slot's hex value by index (0–13, matching `COLOR_SLOTS` order).
    pub fn get(&self, idx: usize) -> &str {
        match idx {
            0 => &self.accent,
            1 => &self.link,
            2 => &self.success,
            3 => &self.notice,
            4 => &self.bg,
            5 => &self.bg_dark,
            6 => &self.text,
            7 => &self.muted_text,
            8 => &self.border,
            9 => &self.unread,
            10 => &self.teal,
            11 => &self.sky,
            12 => &self.pink,
            13 => &self.error,
            _ => "#000000",
        }
    }

    /// Set a color slot by index. No-op for out-of-range indices.
    pub fn set(&mut self, idx: usize, hex: String) {
        match idx {
            0 => self.accent = hex,
            1 => self.link = hex,
            2 => self.success = hex,
            3 => self.notice = hex,
            4 => self.bg = hex,
            5 => self.bg_dark = hex,
            6 => self.text = hex,
            7 => self.muted_text = hex,
            8 => self.border = hex,
            9 => self.unread = hex,
            10 => self.teal = hex,
            11 => self.sky = hex,
            12 => self.pink = hex,
            13 => self.error = hex,
            _ => {}
        }
    }

    /// Serialize to TOML text compatible with `Theme::from_toml_str`.
    pub fn to_toml(&self, name: &str) -> String {
        format!(
            "name = \"{name}\"\n\n[colors]\naccent    = \"{accent}\"\nlink      = \"{link}\"\nsuccess   = \"{success}\"\nnotice    = \"{notice}\"\nbg        = \"{bg}\"\nbg_dark   = \"{bg_dark}\"\ntext      = \"{text}\"\nmuted_text = \"{muted_text}\"\nborder    = \"{border}\"\nunread    = \"{unread}\"\nteal      = \"{teal}\"\nsky       = \"{sky}\"\npink      = \"{pink}\"\nerror     = \"{error}\"\n",
            name = name,
            accent = self.accent,
            link = self.link,
            success = self.success,
            notice = self.notice,
            bg = self.bg,
            bg_dark = self.bg_dark,
            text = self.text,
            muted_text = self.muted_text,
            border = self.border,
            unread = self.unread,
            teal = self.teal,
            sky = self.sky,
            pink = self.pink,
            error = self.error,
        )
    }
}
