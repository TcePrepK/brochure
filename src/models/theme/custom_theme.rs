//! User-created color theme type stored in user data.

use crate::models::theme::custom_theme_colors::CustomThemeColors;
use serde::{Deserialize, Serialize};

/// A user-created color theme stored as 14 named hex strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    /// Unique identifier (monotonically increasing per session).
    pub id: u32,
    /// Display name shown in the theme editor.
    pub name: String,
    /// The 14 color slots for this theme.
    pub colors: CustomThemeColors,
}
