use super::{
    super::{DocumentStatus, Size, Terminal},
    UIComponent,
};
use std::io::Error;

/// Represents the status bar in the editor.
#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    /// Updates the status of the status bar.
    ///
    /// # Arguments
    ///
    /// * `new_status` - The new status to update.
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

    /// Draws the status bar at the specified row.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row to draw the status bar at.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn draw(&mut self, origin_row: usize) -> Result<(), Error> {
        // Assemble the first part of the status bar
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();

        let beginning = format!(
            "{} - {} {}",
            self.current_status.file_name, line_count, modified_indicator
        );

        // Assemble the whole status bar, with the position indicator at the back
        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>remainder_len$}");

        //Only print out the status if it fits. Otherwise write out an empty string to ensure the row is cleared.
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };

        Terminal::print_inverted_row(origin_row, &to_print)?;
        Ok(())
    }
}
