use std::{cmp::min, io::Error};

use super::{super::{command::Edit, Line, Size, Terminal}, UIComponent};


/// Represents the command bar in the editor.
#[derive(Default)]
pub struct CommandBar {
    /// The prompt displayed in the command bar.
    prompt: String,
    /// The value entered in the command bar.
    value: Line,
    /// Indicates whether the command bar needs to be redrawn.
    needs_redraw: bool,
    /// The size of the command bar.
    size: Size,
}

impl CommandBar {
    /// Handles an edit command in the command bar.
    ///
    /// # Arguments
    ///
    /// * `command` - The edit command to handle.
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => self.value.append_char(character),
            Edit::DeleteBackwards => self.value.delete_last(),
            _ => {}
        }
        self.set_needs_redraw(true);
    }

    /// Returns the column position of the caret in the command bar.
    ///
    /// # Returns
    ///
    /// The column position of the caret.
    pub fn caret_position_col(&self) -> usize {
        let max_width = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());
        min(max_width, self.size.width)
    }

    /// Returns the value entered in the command bar as a string.
    ///
    /// # Returns
    ///
    /// The value entered in the command bar.
    pub fn value(&self) -> String {
        self.value.to_string()
    }

    /// Sets the prompt displayed in the command bar.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt to set.
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.set_needs_redraw(true);
    }

    /// Clears the value entered in the command bar.
    pub fn clear_value(&mut self) {
        self.value = Line::default();
        self.set_needs_redraw(true);
    }
}

impl UIComponent for CommandBar {
    /// Sets whether the command bar needs to be redrawn.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` if the command bar needs to be redrawn, `false` otherwise.
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    /// Checks if the command bar needs to be redrawn.
    ///
    /// # Returns
    ///
    /// `true` if the command bar needs to be redrawn, `false` otherwise.
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Sets the size of the command bar.
    ///
    /// # Arguments
    ///
    /// * `size` - The size to set.
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Draws the command bar at the specified row.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row to draw the command bar at.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn draw(&mut self, origin_row: usize) -> Result<(), Error> {
        // This is how much space there is between the right side of the prompt and the edge of the bar
        let area_for_value = self.size.width.saturating_sub(self.prompt.len());
        // We always want to show the left part of the value, therefore the end of the visible range we try to access will be equal to the full width
        let value_end = self.value.width();
        // This should give us the start for the grapheme subrange we want to print out.
        let value_start = value_end.saturating_sub(area_for_value);
        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );
        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };
        Terminal::print_row(origin_row, &to_print)
    }
}
