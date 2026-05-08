/// State for the theme editor: cursors and editing context.
#[derive(Default)]
pub struct ThemeEditorState {
    /// Cursor in the theme editor list (builtins first, then custom themes).
    pub cursor: usize,
    /// Cursor in the color-slot list when editing a custom theme.
    pub color_cursor: usize,
    /// Cursor in the clone-from picker when creating a new custom theme.
    pub clone_cursor: usize,
    /// ID of the custom theme currently being edited or renamed.
    pub editing_id: Option<u32>,
    /// File path typed by the user in ThemeEditorExport / ThemeEditorImport states.
    pub path_input: String,
    /// Text buffer for hex color entry.
    pub hex_input: String,
    /// Cursor position (in chars) within the active theme editor text input field.
    pub input_cursor: usize,
}
