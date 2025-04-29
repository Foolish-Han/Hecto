use super::{
    DocumentStatus, NAME, VERSION,
    command::{Edit, Move},
    terminal::{Position, Size, Terminal},
    uicomponent::UIComponent,
};
use std::{cmp::min, io::Error, usize};
mod buffer;
use buffer::Buffer;
mod line;

/// Represents a location in the text buffer.
#[derive(Default, Clone, Copy)]
pub struct Location {
    /// The index of the grapheme within the line.
    pub grapheme_index: usize,
    /// The index of the line within the buffer.
    pub line_index: usize,
}

/// Represents the view of the text buffer.
#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    // The view always starts at `(0/0)`. The `size` property determines the visible area.
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl View {
    /// Returns the status of the document.
    ///
    /// # Returns
    ///
    /// A `DocumentStatus` instance representing the status of the document.
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_index: self.text_location.line_index,
            file_name: format!("{}", self.buffer.file_info),
            is_modified: self.buffer.dirty,
        }
    }

    // region: file i/o

    /// Loads a file into the buffer.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to load.
    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let buffer = Buffer::load(file_name)?;
        self.buffer = buffer;
        self.set_needs_redraw(true);
        Ok(())
    }

    /// Saves the buffer to a file.
    pub fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()
    }

    // endregion

    // region: command handling

    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::DeleteBackwards => self.delete_backward(),
            Edit::Delete => self.delete(),
            Edit::InsertNewline => self.insert_newline(),
            Edit::Insert(character) => self.insert_char(character),
        }
    }

    pub fn handle_move_command(&mut self, command: Move) {
        let Size { height, .. } = self.size;
        match command {
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::PageUp => self.move_up(height.saturating_sub(1)),
            Move::PageDown => self.move_down(height.saturating_sub(1)),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::StartOfLine => self.move_to_start_of_line(),
            Move::EndOfLine => self.move_to_end_of_line(),
        }
        self.scroll_text_location_into_view();
    }

    // endregion

    // region: Text-editing

    /// Inserts a newline at the current text location.
    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.handle_move_command(Move::Right);
        self.set_needs_redraw(true);
    }

    /// Deletes the character before the current text location.
    fn delete_backward(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.handle_move_command(Move::Left);
            self.delete();
        }
    }

    /// Deletes the character at the current text location.
    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.set_needs_redraw(true);
    }

    /// Inserts a character at the current text location.
    ///
    /// # Arguments
    ///
    /// * `character` - The character to insert.
    fn insert_char(&mut self, character: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.grapheme_count());
        self.buffer.insert_char(character, self.text_location);
        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.grapheme_count());
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            // move right for an added grapheme (should be the regular case)
            self.handle_move_command(Move::Right);
        }
        self.set_needs_redraw(true);
    }

    // endregion

    // region: Rendering

    /// Renders a line of text at the specified position.
    ///
    /// # Arguments
    ///
    /// * `at` - The position to render the line.
    /// * `line_text` - The text to render.
    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }

    /// Builds the welcome message for the editor.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the welcome message.
    ///
    /// # Returns
    ///
    /// A string containing the welcome message.
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);
        // hide the welcome message if it doesn't fit entirely.
        if remaining_width < len {
            return "~".to_string();
        }
        format!("{:1<}{:^remaining_width$}", "~", welcome_message)
    }

    // endregion

    // region: Scrolling

    /// Scrolls the view vertically to the specified position.
    ///
    /// # Arguments
    ///
    /// * `to` - The position to scroll to.
    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    /// Scrolls the view horizontally to the specified position.
    ///
    /// # Arguments
    ///
    /// * `to` - The position to scroll to.
    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    /// Scrolls the text location into view.
    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    // endregion

    // region: Location and Position Handling

    /// Returns the caret position in the view.
    ///
    /// # Returns
    ///
    /// A `Position` instance representing the caret position.
    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    /// Converts the text location to a position in the view.
    ///
    /// # Returns
    ///
    /// A `Position` instance representing the position of the text location.
    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    // endregion

    // region: text location movement

    /// Moves the text location up by the specified number of steps.
    ///
    /// # Arguments
    ///
    /// * `step` - The number of steps to move up.
    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    /// Moves the text location down by the specified number of steps.
    ///
    /// # Arguments
    ///
    /// * `step` - The number of steps to move down.
    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_line();
        self.snap_to_valid_grapheme();
    }

    // clippy::arithmetic_side_effects: This function performs arithmetic calculations
    // after explicitly checking that the target value will be within bounds.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.grapheme_count());
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    // clippy::arithmetic_side_effects: This function performs arithmetic calculations
    // after explicitly checking that the target value will be within bounds.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    /// Moves the text location to the start of the current line.
    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    /// Moves the text location to the end of the current line.
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.grapheme_count());
    }

    /// Ensures the text location points to a valid grapheme index by snapping it to the leftmost grapheme if appropriate.
    /// Doesn't trigger scrolling.
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    /// Ensures the text location points to a valid line index by snapping it to the bottommost line if appropriate.
    /// Doesn't trigger scrolling.
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }
    // endregion
}

impl UIComponent for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        let Size { height, width } = self.size;
        let end_y = origin_y.saturating_add(height);

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit too far up or down
        #[allow(clippy::integer_division)]
        let top_third = height / 3;
        let scroll_top = self.scroll_offset.row;

        for current_row in origin_y..end_y {
            // to get the correct line index, we have to take current_row (the absolute row on screen),
            // subtract origin_y to get the current row relative to the view (ranging from 0 to self.size.height)
            // and add the scroll offset.
            let line_idx = current_row
                .saturating_sub(origin_y)
                .saturating_add(scroll_top);
            if let Some(line) = self.buffer.lines.get(line_idx) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right))?;
            } else if current_row == top_third && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        Ok(())
    }
}
