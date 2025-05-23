/// Represents the status of a document.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct DocumentStatus {
    /// The total number of lines in the document.
    pub total_lines: usize,
    /// The current line index in the document.
    pub current_line_idx: usize,
    /// Indicates whether the document has been modified.
    pub is_modified: bool,
    /// The name of the file associated with the document.
    pub file_name: String,
}

impl DocumentStatus {
    /// Returns a string indicating whether the document has been modified.
    ///
    /// # Returns
    ///
    /// A string indicating whether the document has been modified.
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }

    /// Returns a string representing the total number of lines in the document.
    ///
    /// # Returns
    ///
    /// A string representing the total number of lines in the document.
    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    /// Returns a string representing the current position in the document.
    ///
    /// # Returns
    ///
    /// A string representing the current position in the document.
    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line_idx.saturating_add(1),
            self.total_lines
        )
    }
}
