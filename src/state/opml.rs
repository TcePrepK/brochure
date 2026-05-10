//! State for the OPML import/export file-path input screens.

/// Mutable state for OPMLImportPath and OPMLExportPath input screens.
#[derive(Default)]
pub struct OpmlState {
    /// File path typed by the user.
    pub path_input: String,
    /// Cursor position (in chars) within the path input.
    pub input_cursor: usize,
}
