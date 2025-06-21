//! # Status Bar Component
//!
//! This module implements the status bar component that displays document
//! information including filename, line count, modification status, and
//! current cursor position. The status bar appears as an inverted row
//! near the bottom of the editor interface.

use crate::prelude::*;

use std::io::Error;

use super::{
    super::{DocumentStatus, Size, Terminal},
    UIComponent,
};

/// Status bar component for displaying document information
///
/// The StatusBar shows important document metadata in a horizontal bar
/// with inverted colors. It displays:
/// - Document filename
/// - Total line count
/// - Modification status (if unsaved changes exist)
/// - Current cursor position (current line / total lines)
///
/// The information is formatted to fit within the available width,
/// with the position indicator right-aligned.
#[derive(Default)]
pub struct StatusBar {
    /// Current document status information
    current_status: DocumentStatus,
    /// Whether the component needs redrawing
    needs_redraw: bool,
    /// Component dimensions
    size: Size,
}

impl StatusBar {
    /// Updates the status bar with new document information
    ///
    /// This method compares the new status with the current status and
    /// only triggers a redraw if the information has actually changed.
    /// This optimization prevents unnecessary rendering operations.
    ///
    /// # Arguments
    ///
    /// * `new_status` - The updated document status information
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if self.current_status != new_status {
            self.current_status = new_status;
            self.set_needs_redraw(true);
        }
    }
}
impl UIComponent for StatusBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Renders the status bar with document information
    ///
    /// This method constructs and displays the status bar content using inverted
    /// colors. The layout consists of:
    /// - Left side: filename, line count, and modification indicator
    /// - Right side: current position indicator
    ///
    /// The content is formatted to fit within the available width. If the
    /// constructed status line would be too long, an empty line is displayed
    /// instead to prevent layout issues.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row where the status bar should be rendered
    ///
    /// # Returns
    ///
    /// `Ok(())` on successful rendering, or an `Error` if terminal operations fail
    fn draw(&mut self, origin_row: RowIdx) -> Result<(), Error> {
        // Construct the left side of the status bar
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let beginning = format!(
            "{} - {} {}",
            self.current_status.file_name, line_count, modified_indicator
        );

        // Construct the right side (position indicator)
        let position_indicator = self.current_status.position_indicator_to_string();

        // Calculate space available for right-alignment
        let remainder_len = self.size.width.saturating_sub(beginning.len());

        // Format the complete status line with right-aligned position
        let status = format!("{beginning}{position_indicator:>remainder_len$}");

        // Only display if it fits within the available width
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };

        // Render with inverted colors
        Terminal::print_inverted_row(origin_row, &to_print)?;
        Ok(())
    }
}
