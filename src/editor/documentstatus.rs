//! # Document Status Module
//!
//! This module defines the DocumentStatus structure which represents the current
//! state of a document being edited, including metadata such as line count,
//! current position, modification status, and filename.

/// Represents the current status and metadata of a document
///
/// DocumentStatus contains all the information needed to display document
/// status in the UI, including the total number of lines, current cursor position,
/// whether the document has been modified, and the filename.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct DocumentStatus {
    /// Total number of lines in the document
    pub total_lines: usize,
    /// Current line index (0-based) where the cursor is positioned
    pub current_line_idx: usize,
    /// Whether the document has unsaved modifications
    pub is_modified: bool,
    /// Name of the file, or a placeholder for new documents
    pub file_name: String,
}

impl DocumentStatus {
    /// Returns a string indicating whether the document has been modified
    ///
    /// This method returns "(modified)" if the document has unsaved changes,
    /// or an empty string if the document is unmodified.
    ///
    /// # Returns
    ///
    /// A String containing the modification indicator
    ///
    /// # Examples
    ///
    /// ```
    /// let mut status = DocumentStatus::default();
    /// assert_eq!(status.modified_indicator_to_string(), "");
    ///
    /// status.is_modified = true;
    /// assert_eq!(status.modified_indicator_to_string(), "(modified)");
    /// ```
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }

    /// Returns a string representation of the total line count
    ///
    /// This method formats the total number of lines for display in the UI.
    ///
    /// # Returns
    ///
    /// A String containing the formatted line count
    ///
    /// # Examples
    ///
    /// ```
    /// let status = DocumentStatus {
    ///     total_lines: 42,
    ///     ..Default::default()
    /// };
    /// assert_eq!(status.line_count_to_string(), "42 lines");
    /// ```
    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    /// Returns a string representation of the current position within the document
    ///
    /// This method formats the current line position as "current/total" for
    /// display in the status bar. The current line is displayed as 1-based
    /// for user-friendliness.
    ///
    /// # Returns
    ///
    /// A String containing the formatted position indicator
    ///
    /// # Examples
    ///
    /// ```
    /// let status = DocumentStatus {
    ///     current_line_idx: 4,  // 0-based
    ///     total_lines: 10,
    ///     ..Default::default()
    /// };
    /// assert_eq!(status.position_indicator_to_string(), "5/10");
    /// ```
    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line_idx.saturating_add(1),
            self.total_lines
        )
    }
}
