//! Theme definitions: color palette struct, built-in themes, and custom TOML loading.

use ratatui::style::Color;

/// Metadata for each color slot: `(field_name, short_label)` in the order used by
/// [`crate::models::CustomThemeColors::get`] / [`crate::models::CustomThemeColors::set`].
pub const COLOR_SLOTS: &[(&str, &str)] = &[
    ("mauve", "accent / focused border"),
    ("blue", "links / highlights"),
    ("green", "success / read indicator"),
    ("peach", "section headers / warnings"),
    ("base", "main background"),
    ("mantle", "darkest background"),
    ("text", "primary foreground"),
    ("subtext0", "secondary / muted text"),
    ("surface0", "unfocused borders"),
    ("yellow", "warnings / stars / unread"),
    ("teal", "teal accent"),
    ("sky", "lighter blue accent"),
    ("pink", "pink accent"),
    ("red", "errors / delete actions"),
];

/// Full color palette for the application UI.
///
/// Every named slot maps to one semantic role (e.g. `mauve` = focused border/accent,
/// `surface0` = unfocused border, `base` = main background). Built-in constructors
/// return ready-to-use palettes; `from_toml_str` loads custom user themes.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme display name (shown in the theme picker).
    pub name: String,
    /// Accent / focused-border color.
    pub mauve: Color,
    /// Link / highlight color.
    pub blue: Color,
    /// Success / read-indicator color.
    pub green: Color,
    /// Section-header / warning color.
    pub peach: Color,
    /// Main background.
    pub base: Color,
    /// Darkest background (tab bar, footer).
    pub mantle: Color,
    /// Primary foreground text.
    pub text: Color,
    /// Secondary / muted text.
    pub subtext0: Color,
    /// Unfocused border / muted element color.
    pub surface0: Color,
    /// Warning / star / unread accent.
    pub yellow: Color,
    /// Teal accent variant.
    pub teal: Color,
    /// Sky / lighter blue accent.
    pub sky: Color,
    /// Pink accent variant.
    pub pink: Color,
    /// Error / delete action color.
    pub red: Color,
}

impl Theme {
    /// Colors cycled by category ID in the sidebar (8-element fixed array).
    pub fn category_colors(&self) -> [Color; 8] {
        [
            self.mauve,
            self.blue,
            self.green,
            self.peach,
            self.yellow,
            self.teal,
            self.sky,
            self.pink,
        ]
    }

    /// Catppuccin Mocha — the original brochure palette.
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: String::from("Catppuccin Mocha"),
            mauve: Color::Rgb(203, 166, 247),
            blue: Color::Rgb(137, 180, 250),
            green: Color::Rgb(166, 227, 161),
            peach: Color::Rgb(250, 179, 135),
            base: Color::Rgb(30, 30, 46),
            mantle: Color::Rgb(24, 24, 37),
            text: Color::Rgb(205, 214, 244),
            subtext0: Color::Rgb(166, 173, 200),
            surface0: Color::Rgb(49, 50, 68),
            yellow: Color::Rgb(249, 226, 175),
            teal: Color::Rgb(148, 226, 213),
            sky: Color::Rgb(137, 220, 235),
            pink: Color::Rgb(245, 194, 231),
            red: Color::Rgb(243, 139, 168),
        }
    }

    /// Gruvbox Dark — warm retro palette.
    pub fn gruvbox_dark() -> Self {
        Self {
            name: String::from("Gruvbox Dark"),
            mauve: Color::Rgb(211, 134, 155),
            blue: Color::Rgb(131, 165, 152),
            green: Color::Rgb(184, 187, 38),
            peach: Color::Rgb(254, 128, 25),
            base: Color::Rgb(40, 40, 40),
            mantle: Color::Rgb(29, 32, 33),
            text: Color::Rgb(235, 219, 178),
            subtext0: Color::Rgb(168, 153, 132),
            surface0: Color::Rgb(60, 56, 54),
            yellow: Color::Rgb(250, 189, 47),
            teal: Color::Rgb(142, 192, 124),
            sky: Color::Rgb(131, 165, 152),
            pink: Color::Rgb(211, 134, 155),
            red: Color::Rgb(251, 73, 52),
        }
    }

    /// Dracula — high-contrast purple/pink palette.
    pub fn dracula() -> Self {
        Self {
            name: String::from("Dracula"),
            mauve: Color::Rgb(189, 147, 249),
            blue: Color::Rgb(98, 114, 164),
            green: Color::Rgb(80, 250, 123),
            peach: Color::Rgb(255, 184, 108),
            base: Color::Rgb(40, 42, 54),
            mantle: Color::Rgb(25, 26, 33),
            text: Color::Rgb(248, 248, 242),
            subtext0: Color::Rgb(98, 114, 164),
            surface0: Color::Rgb(68, 71, 90),
            yellow: Color::Rgb(241, 250, 140),
            teal: Color::Rgb(139, 233, 253),
            sky: Color::Rgb(139, 233, 253),
            pink: Color::Rgb(255, 121, 198),
            red: Color::Rgb(255, 85, 85),
        }
    }

    /// Nord — cool arctic palette.
    pub fn nord() -> Self {
        Self {
            name: String::from("Nord"),
            mauve: Color::Rgb(180, 142, 173),
            blue: Color::Rgb(129, 161, 193),
            green: Color::Rgb(163, 190, 140),
            peach: Color::Rgb(208, 135, 112),
            base: Color::Rgb(46, 52, 64),
            mantle: Color::Rgb(36, 41, 51),
            text: Color::Rgb(236, 239, 244),
            subtext0: Color::Rgb(216, 222, 233),
            surface0: Color::Rgb(59, 66, 82),
            yellow: Color::Rgb(235, 203, 139),
            teal: Color::Rgb(143, 188, 187),
            sky: Color::Rgb(136, 192, 208),
            pink: Color::Rgb(180, 142, 173),
            red: Color::Rgb(191, 97, 106),
        }
    }

    /// GNOME Adwaita Dark — clean GTK palette.
    pub fn gnome() -> Self {
        Self {
            name: String::from("GNOME"),
            mauve: Color::Rgb(145, 65, 172),
            blue: Color::Rgb(53, 132, 228),
            green: Color::Rgb(38, 162, 105),
            peach: Color::Rgb(230, 97, 0),
            base: Color::Rgb(30, 30, 30),
            mantle: Color::Rgb(20, 20, 20),
            text: Color::Rgb(255, 255, 255),
            subtext0: Color::Rgb(154, 153, 150),
            surface0: Color::Rgb(48, 48, 48),
            yellow: Color::Rgb(229, 165, 10),
            teal: Color::Rgb(33, 144, 164),
            sky: Color::Rgb(99, 160, 212),
            pink: Color::Rgb(192, 97, 203),
            red: Color::Rgb(224, 27, 36),
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
            mauve: p("mauve", &c.mauve)?,
            blue: p("blue", &c.blue)?,
            green: p("green", &c.green)?,
            peach: p("peach", &c.peach)?,
            base: p("base", &c.base)?,
            mantle: p("mantle", &c.mantle)?,
            text: p("text", &c.text)?,
            subtext0: p("subtext0", &c.subtext0)?,
            surface0: p("surface0", &c.surface0)?,
            yellow: p("yellow", &c.yellow)?,
            teal: p("teal", &c.teal)?,
            sky: p("sky", &c.sky)?,
            pink: p("pink", &c.pink)?,
            red: p("red", &c.red)?,
        })
    }

    /// Convert this runtime theme into [`crate::models::CustomThemeColors`] hex strings.
    ///
    /// Used when cloning a built-in theme as the starting point for a new custom theme.
    pub fn to_custom_colors(&self) -> crate::models::CustomThemeColors {
        crate::models::CustomThemeColors {
            mauve: Self::color_to_hex(self.mauve),
            blue: Self::color_to_hex(self.blue),
            green: Self::color_to_hex(self.green),
            peach: Self::color_to_hex(self.peach),
            base: Self::color_to_hex(self.base),
            mantle: Self::color_to_hex(self.mantle),
            text: Self::color_to_hex(self.text),
            subtext0: Self::color_to_hex(self.subtext0),
            surface0: Self::color_to_hex(self.surface0),
            yellow: Self::color_to_hex(self.yellow),
            teal: Self::color_to_hex(self.teal),
            sky: Self::color_to_hex(self.sky),
            pink: Self::color_to_hex(self.pink),
            red: Self::color_to_hex(self.red),
        }
    }

    /// Parse a custom theme from TOML source text.
    ///
    /// The TOML format is:
    /// ```toml
    /// name = "My Theme"
    ///
    /// [colors]
    /// mauve   = "#cba6f7"
    /// blue    = "#89b4fa"
    /// green   = "#a6e3a1"
    /// peach   = "#fab387"
    /// base    = "#1e1e2e"
    /// mantle  = "#181825"
    /// text    = "#cdd6f4"
    /// subtext0 = "#a6adc8"
    /// surface0 = "#313244"
    /// yellow  = "#f9e2af"
    /// teal    = "#94e2d5"
    /// sky     = "#89dceb"
    /// pink    = "#f5c2e7"
    /// red     = "#f38ba8"
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
            mauve: parse("mauve")?,
            blue: parse("blue")?,
            green: parse("green")?,
            peach: parse("peach")?,
            base: parse("base")?,
            mantle: parse("mantle")?,
            text: parse("text")?,
            subtext0: parse("subtext0")?,
            surface0: parse("surface0")?,
            yellow: parse("yellow")?,
            teal: parse("teal")?,
            sky: parse("sky")?,
            pink: parse("pink")?,
            red: parse("red")?,
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
        let t = Theme::catppuccin_mocha();
        assert_eq!(t.mauve, Color::Rgb(203, 166, 247));
        assert_eq!(t.name, "Catppuccin Mocha");
    }

    #[test]
    fn builtin_lookup_by_slug() {
        assert!(Theme::builtin("catppuccin-mocha").is_some());
        assert!(Theme::builtin("gruvbox-dark").is_some());
        assert!(Theme::builtin("dracula").is_some());
        assert!(Theme::builtin("nord").is_some());
        assert!(Theme::builtin("gnome").is_some());
        assert!(Theme::builtin("unknown").is_none());
    }

    #[test]
    fn category_colors_returns_8_entries() {
        let t = Theme::catppuccin_mocha();
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
mauve    = "#cba6f7"
blue     = "#89b4fa"
green    = "#a6e3a1"
peach    = "#fab387"
base     = "#1e1e2e"
mantle   = "#181825"
text     = "#cdd6f4"
subtext0 = "#a6adc8"
surface0 = "#313244"
yellow   = "#f9e2af"
teal     = "#94e2d5"
sky      = "#89dceb"
pink     = "#f5c2e7"
red      = "#f38ba8"
"##;
        let t = Theme::from_toml_str(src).unwrap();
        assert_eq!(t.name, "Test");
        assert_eq!(t.mauve, Color::Rgb(203, 166, 247));
    }

    #[test]
    fn from_toml_str_missing_color_errors() {
        let src = r##"
name = "Broken"
[colors]
mauve = "#cba6f7"
"##;
        assert!(Theme::from_toml_str(src).is_err());
    }
}
