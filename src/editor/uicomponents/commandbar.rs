//! # Command Bar Component
//!
//! This module implements the command bar component that handles user input
//! during interactive prompts such as save-as operations and search queries.
//! The command bar displays a prompt message and allows text input with
//! basic editing capabilities.

use std::{cmp::min, io::Error};

use super::{
    super::{Line, Size, Terminal, command::Edit},
    UIComponent,
};

/// Command bar component for interactive user input
///
/// The CommandBar provides a text input interface that appears at the bottom
/// of the editor during various prompt operations. It consists of:
/// - A prompt string that describes what the user should enter
/// - An input area where the user can type and edit text
/// - Visual feedback showing the current input state
///
/// ## Supported Operations
///
/// - Character insertion and deletion
/// - Horizontal scrolling for long input
/// - Cursor position tracking
/// - Value retrieval for processing user input
///
/// ## Usage
///
/// The command bar is typically used in these scenarios:
/// - Save-as prompts (asking for a filename)
/// - Search prompts (asking for search terms)
/// - Any other operation requiring text input from the user
#[derive(Default)]
pub struct CommandBar {
    /// The prompt text displayed to the user
    prompt: String,
    /// The current input value as a Line (supports Unicode)
    value: Line,
    /// Whether the component needs redrawing
    needs_redraw: bool,
    /// Component dimensions
    size: Size,
}

impl CommandBar {
    /// Handles editing commands for the input area
    ///
    /// This method processes edit commands that modify the input text,
    /// such as character insertion and deletion. It automatically marks
    /// the component for redraw after any modification.
    ///
    /// # Arguments
    ///
    /// * `command` - The edit command to process
    ///
    /// # Supported Commands
    ///
    /// - `Insert(char)`: Appends a character to the input
    /// - `DeleteBackward`: Removes the last character
    /// - Other edit commands are ignored
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => self.value.append_char(character),
            Edit::DeleteBackward => self.value.delete_last(),
            _ => {}, // Other edit commands are not supported in command bar
        }
        self.set_needs_redraw(true);
    }

    /// Calculates the cursor position for display
    ///
    /// This method determines where the cursor should be positioned on the
    /// screen, taking into account the prompt length and current input length.
    /// The position is clamped to the available width.
    ///
    /// # Returns
    ///
    /// The column position where the cursor should be displayed
    pub fn caret_position_col(&self) -> usize {
        let max_width = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());
        min(max_width, self.size.width)
    }

    /// Returns the current input value as a string
    ///
    /// # Returns
    ///
    /// The current text input from the user
    pub fn value(&self) -> String {
        self.value.to_string()
    }

    /// Sets the prompt text and triggers a redraw
    ///
    /// # Arguments
    ///
    /// * `prompt` - The new prompt text to display
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.set_needs_redraw(true);
    }

    /// Clears the input value and triggers a redraw
    pub fn clear_value(&mut self) {
        self.value = Line::default();
        self.set_needs_redraw(true);
    }
}
impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn draw(&mut self, origin_row: usize) -> Result<(), Error> {
        let area_for_value = self.size.width.saturating_sub(self.prompt.len());
        let value_end = self.value.width();
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
