//! Theme definitions: color palette struct, built-in themes, and custom TOML loading.

use ratatui::style::Color;

/// Metadata for each color slot: `(field_name, short_label)` in the order used by
/// [`crate::models::CustomThemeColors::get`] / [`crate::models::CustomThemeColors::set`].
pub const COLOR_SLOTS: &[(&str, &str)] = &[
    ("accent", "primary accent / focused border"),
    ("link", "links / highlights"),
    ("success", "success / read indicator"),
    ("notice", "section headers / warnings"),
    ("bg", "main background"),
    ("bg_dark", "darkest background"),
    ("text", "primary foreground"),
    ("muted_text", "secondary / muted text"),
    ("border", "unfocused borders"),
    ("unread", "warnings / stars / unread"),
    ("teal", "teal accent"),
    ("sky", "sky / lighter accent"),
    ("pink", "pink accent"),
    ("error", "errors / delete actions"),
];

/// Full color palette for the application UI.
///
/// Every named slot maps to one semantic role (e.g. `accent` = focused border,
/// `border` = unfocused border, `bg` = main background). Built-in constructors
/// return ready-to-use palettes; `from_toml_str` loads custom user themes.
#[derive(Debug, Clone)]
pub struct ColorTheme {
    /// Theme display name (shown in the theme picker).
    pub name: String,
    /// Focused-border / primary accent.
    pub accent: Color,
    /// Highlight.
    pub link: Color,
    /// Read-indicator / positive.
    pub success: Color,
    /// Section-header / warning.
    pub notice: Color,
    /// Main background.
    pub bg: Color,
    /// Darkest background (tab bar, footer).
    pub bg_dark: Color,
    /// Primary foreground text.
    pub text: Color,
    /// Secondary / muted text.
    pub muted_text: Color,
    /// Unfocused border / muted element.
    pub border: Color,
    /// Warning / star / unread accent.
    pub unread: Color,
    /// Teal accent variant.
    pub teal: Color,
    /// Sky / lighter blue accent.
    pub sky: Color,
    /// Pink accent variant.
    pub pink: Color,
    /// Error / delete action.
    pub error: Color,
}

impl ColorTheme {
    /// Colors cycled by category ID in the sidebar (8-element fixed array).
    pub fn category_colors(&self) -> [Color; 8] {
        [
            self.accent,
            self.link,
            self.success,
            self.notice,
            self.unread,
            self.teal,
            self.sky,
            self.pink,
        ]
    }

    /// Catppuccin Mocha — the original brochure palette.
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: String::from("Catppuccin Mocha"),
            accent: Color::Rgb(203, 166, 247),
            link: Color::Rgb(137, 180, 250),
            success: Color::Rgb(166, 227, 161),
            notice: Color::Rgb(250, 179, 135),
            bg: Color::Rgb(30, 30, 46),
            bg_dark: Color::Rgb(24, 24, 37),
            text: Color::Rgb(205, 214, 244),
            muted_text: Color::Rgb(166, 173, 200),
            border: Color::Rgb(49, 50, 68),
            unread: Color::Rgb(249, 226, 175),
            teal: Color::Rgb(148, 226, 213),
            sky: Color::Rgb(137, 220, 235),
            pink: Color::Rgb(245, 194, 231),
            error: Color::Rgb(243, 139, 168),
        }
    }

    /// Gruvbox Dark — warm retro palette.
    pub fn gruvbox_dark() -> Self {
        Self {
            name: String::from("Gruvbox Dark"),
            accent: Color::Rgb(211, 134, 155),
            link: Color::Rgb(131, 165, 152),
            success: Color::Rgb(184, 187, 38),
            notice: Color::Rgb(254, 128, 25),
            bg: Color::Rgb(40, 40, 40),
            bg_dark: Color::Rgb(29, 32, 33),
            text: Color::Rgb(235, 219, 178),
            muted_text: Color::Rgb(168, 153, 132),
            border: Color::Rgb(60, 56, 54),
            unread: Color::Rgb(250, 189, 47),
            teal: Color::Rgb(142, 192, 124),
            sky: Color::Rgb(131, 165, 152),
            pink: Color::Rgb(211, 134, 155),
            error: Color::Rgb(251, 73, 52),
        }
    }

    /// Dracula — high-contrast purple/pink palette.
    pub fn dracula() -> Self {
        Self {
            name: String::from("Dracula"),
            accent: Color::Rgb(189, 147, 249),
            link: Color::Rgb(98, 114, 164),
            success: Color::Rgb(80, 250, 123),
            notice: Color::Rgb(255, 184, 108),
            bg: Color::Rgb(40, 42, 54),
            bg_dark: Color::Rgb(25, 26, 33),
            text: Color::Rgb(248, 248, 242),
            muted_text: Color::Rgb(98, 114, 164),
            border: Color::Rgb(68, 71, 90),
            unread: Color::Rgb(241, 250, 140),
            teal: Color::Rgb(139, 233, 253),
            sky: Color::Rgb(139, 233, 253),
            pink: Color::Rgb(255, 121, 198),
            error: Color::Rgb(255, 85, 85),
        }
    }

    /// Nord — cool arctic palette.
    pub fn nord() -> Self {
        Self {
            name: String::from("Nord"),
            accent: Color::Rgb(180, 142, 173),
            link: Color::Rgb(129, 161, 193),
            success: Color::Rgb(163, 190, 140),
            notice: Color::Rgb(208, 135, 112),
            bg: Color::Rgb(46, 52, 64),
            bg_dark: Color::Rgb(36, 41, 51),
            text: Color::Rgb(236, 239, 244),
            muted_text: Color::Rgb(216, 222, 233),
            border: Color::Rgb(59, 66, 82),
            unread: Color::Rgb(235, 203, 139),
            teal: Color::Rgb(143, 188, 187),
            sky: Color::Rgb(136, 192, 208),
            pink: Color::Rgb(180, 142, 173),
            error: Color::Rgb(191, 97, 106),
        }
    }

    /// GNOME Adwaita Dark — clean GTK palette.
    pub fn gnome() -> Self {
        Self {
            name: String::from("GNOME"),
            accent: Color::Rgb(53, 132, 228),
            link: Color::Rgb(98, 160, 234),
            success: Color::Rgb(38, 162, 105),
            notice: Color::Rgb(230, 97, 0),
            bg: Color::Rgb(30, 30, 30),
            bg_dark: Color::Rgb(20, 20, 20),
            text: Color::Rgb(255, 255, 255),
            muted_text: Color::Rgb(154, 153, 150),
            border: Color::Rgb(48, 48, 48),
            unread: Color::Rgb(229, 165, 10),
            teal: Color::Rgb(28, 113, 216),
            sky: Color::Rgb(153, 193, 241),
            pink: Color::Rgb(192, 97, 203),
            error: Color::Rgb(224, 27, 36),
        }
    }

    /// Returns the built-in theme matching `name`, or `None` if not found.
    ///
    /// Recognised names: `"catppuccin-mocha"`, `"gruvbox-dark"`, `"dracula"`, `"nord"`, `"gnome"`.
    pub fn builtin(name: &str) -> Option<Self> {
        match name {
            "catppuccin-mocha" => Some(Self::catppuccin_mocha()),
            "gruvbox-dark" => Some(Self::gruvbox_dark()),
            "dracula" => Some(Self::dracula()),
            "nord" => Some(Self::nord()),
            "gnome" => Some(Self::gnome()),
            _ => None,
        }
    }

    /// Returns the slug (persisted key) for a built-in theme display name.
    pub fn slug(display_name: &str) -> &'static str {
        match display_name {
            "Catppuccin Mocha" => "catppuccin-mocha",
            "Gruvbox Dark" => "gruvbox-dark",
            "Dracula" => "dracula",
            "Nord" => "nord",
            "GNOME" => "gnome",
            _ => "custom",
        }
    }

    /// All built-in theme display names, in picker order.
    pub fn builtin_names() -> &'static [&'static str] {
        &[
            "Catppuccin Mocha",
            "Gruvbox Dark",
            "Dracula",
            "Nord",
            "GNOME",
        ]
    }

    /// Convert a `Color::Rgb` value to a `#rrggbb` hex string.
    pub fn color_to_hex(color: Color) -> String {
        match color {
            Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
            _ => "#000000".to_string(),
        }
    }

    /// Build a runtime `Theme` from a stored [`crate::models::CustomTheme`].
    pub fn from_custom_theme(ct: &crate::models::CustomTheme) -> anyhow::Result<Self> {
        use anyhow::Context as _;
        let c = &ct.colors;
        let p = |key: &str, hex: &str| -> anyhow::Result<Color> {
            parse_hex(hex).with_context(|| format!("invalid hex for {key}: {hex}"))
        };
        Ok(Self {
            name: ct.name.clone(),
            accent: p("accent", &c.accent)?,
            link: p("link", &c.link)?,
            success: p("success", &c.success)?,
            notice: p("notice", &c.notice)?,
            bg: p("bg", &c.bg)?,
            bg_dark: p("bg_dark", &c.bg_dark)?,
            text: p("text", &c.text)?,
            muted_text: p("muted_text", &c.muted_text)?,
            border: p("border", &c.border)?,
            unread: p("unread", &c.unread)?,
            teal: p("teal", &c.teal)?,
            sky: p("sky", &c.sky)?,
            pink: p("pink", &c.pink)?,
            error: p("error", &c.error)?,
        })
    }

    /// Convert this runtime theme into [`crate::models::CustomThemeColors`] hex strings.
    ///
    /// Used when cloning a built-in theme as the starting point for a new custom theme.
    pub fn to_custom_colors(&self) -> crate::models::CustomThemeColors {
        crate::models::CustomThemeColors {
            accent: Self::color_to_hex(self.accent),
            link: Self::color_to_hex(self.link),
            success: Self::color_to_hex(self.success),
            notice: Self::color_to_hex(self.notice),
            bg: Self::color_to_hex(self.bg),
            bg_dark: Self::color_to_hex(self.bg_dark),
            text: Self::color_to_hex(self.text),
            muted_text: Self::color_to_hex(self.muted_text),
            border: Self::color_to_hex(self.border),
            unread: Self::color_to_hex(self.unread),
            teal: Self::color_to_hex(self.teal),
            sky: Self::color_to_hex(self.sky),
            pink: Self::color_to_hex(self.pink),
            error: Self::color_to_hex(self.error),
        }
    }

    /// Parse a custom theme from TOML source text.
    ///
    /// The TOML format is:
    /// ```toml
    /// name = "My Theme"
    ///
    /// [colors]
    /// accent    = "#cba6f7"
    /// link      = "#89b4fa"
    /// success   = "#a6e3a1"
    /// notice    = "#fab387"
    /// bg        = "#1e1e2e"
    /// bg_dark   = "#181825"
    /// text      = "#cdd6f4"
    /// muted_text = "#a6adc8"
    /// border    = "#313244"
    /// unread    = "#f9e2af"
    /// teal      = "#94e2d5"
    /// sky       = "#89dceb"
    /// pink      = "#f5c2e7"
    /// error     = "#f38ba8"
    /// ```
    pub fn from_toml_str(src: &str) -> anyhow::Result<Self> {
        use anyhow::Context as _;

        let table: toml::Table = toml::from_str(src).context("invalid TOML")?;
        let name = table
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Custom")
            .to_string();
        let colors = table
            .get("colors")
            .and_then(|v| v.as_table())
            .context("[colors] table missing")?;

        let parse = |key: &str| -> anyhow::Result<Color> {
            let hex = colors
                .get(key)
                .and_then(|v| v.as_str())
                .with_context(|| format!("missing color: {key}"))?;
            parse_hex(hex).with_context(|| format!("invalid hex for {key}: {hex}"))
        };

        Ok(Self {
            name,
            accent: parse("accent")?,
            link: parse("link")?,
            success: parse("success")?,
            notice: parse("notice")?,
            bg: parse("bg")?,
            bg_dark: parse("bg_dark")?,
            text: parse("text")?,
            muted_text: parse("muted_text")?,
            border: parse("border")?,
            unread: parse("unread")?,
            teal: parse("teal")?,
            sky: parse("sky")?,
            pink: parse("pink")?,
            error: parse("error")?,
        })
    }
}

/// Parse a CSS hex color string (`#rrggbb`) into a `Color::Rgb`.
#[allow(dead_code)]
fn parse_hex(hex: &str) -> anyhow::Result<Color> {
    let hex = hex.trim_start_matches('#');
    anyhow::ensure!(hex.len() == 6, "expected 6 hex digits, got {}", hex.len());
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok(Color::Rgb(r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_catppuccin_mocha_has_correct_mauve() {
        let t = ColorTheme::catppuccin_mocha();
        assert_eq!(t.accent, Color::Rgb(203, 166, 247));
        assert_eq!(t.name, "Catppuccin Mocha");
    }

    #[test]
    fn builtin_lookup_by_slug() {
        assert!(ColorTheme::builtin("catppuccin-mocha").is_some());
        assert!(ColorTheme::builtin("gruvbox-dark").is_some());
        assert!(ColorTheme::builtin("dracula").is_some());
        assert!(ColorTheme::builtin("nord").is_some());
        assert!(ColorTheme::builtin("gnome").is_some());
        assert!(ColorTheme::builtin("unknown").is_none());
    }

    #[test]
    fn category_colors_returns_8_entries() {
        let t = ColorTheme::catppuccin_mocha();
        assert_eq!(t.category_colors().len(), 8);
    }

    #[test]
    fn parse_hex_valid() {
        assert_eq!(parse_hex("#cba6f7").unwrap(), Color::Rgb(203, 166, 247));
        assert_eq!(parse_hex("cba6f7").unwrap(), Color::Rgb(203, 166, 247));
    }

    #[test]
    fn parse_hex_invalid_length() {
        assert!(parse_hex("#abc").is_err());
    }

    #[test]
    fn from_toml_str_valid() {
        let src = r##"
name = "Test"
[colors]
accent    = "#cba6f7"
link      = "#89b4fa"
success   = "#a6e3a1"
notice    = "#fab387"
bg        = "#1e1e2e"
bg_dark   = "#181825"
text      = "#cdd6f4"
muted_text = "#a6adc8"
border    = "#313244"
unread    = "#f9e2af"
teal      = "#94e2d5"
sky       = "#89dceb"
pink      = "#f5c2e7"
error     = "#f38ba8"
"##;
        let t = ColorTheme::from_toml_str(src).unwrap();
        assert_eq!(t.name, "Test");
        assert_eq!(t.accent, Color::Rgb(203, 166, 247));
    }

    #[test]
    fn from_toml_str_missing_color_errors() {
        let src = r##"
name = "Broken"
[colors]
accent = "#cba6f7"
"##;
        assert!(ColorTheme::from_toml_str(src).is_err());
    }
}
