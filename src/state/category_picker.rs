//! State for the CategoryPicker modal (save-to-category flow).

use crate::models::AppState;

/// All mutable state for the CategoryPicker modal.
pub struct CategoryPickerState {
    /// Cursor row in the picker list (0..categories + 2).
    pub cursor: usize,
    /// True when the "New category..." entry is active and user is typing.
    pub new_mode: bool,
    /// Text buffer for the new-category name input inside the picker.
    pub input: String,
    /// State to return to when the picker closes (ArticleList or ArticleDetail).
    pub return_state: AppState,
    /// Cursor position (in chars) within the new-category text input.
    pub input_cursor: usize,
}

impl Default for CategoryPickerState {
    fn default() -> Self {
        Self {
            cursor: 0,
            new_mode: false,
            input: String::new(),
            return_state: AppState::ArticleList,
            input_cursor: 0,
        }
    }
}
