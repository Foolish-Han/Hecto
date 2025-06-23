//! Main text editing view component for the Hecto text editor.
//!
//! This module provides the `View` struct, which is the primary component responsible for
//! text editing, display, and user interaction. The view manages document content through
//! a buffer, handles cursor positioning, scrolling, search functionality, and renders
//! the text content to the terminal.
//!
//! # Architecture
//!
//! The view component is structured as follows:
//! - **Buffer Management**: Text content storage and manipulation through the `Buffer` type
//! - **Location Tracking**: Cursor position and scroll offset management via `Location` and `Position`
//! - **Search System**: Text search functionality with forward/backward navigation
//! - **Rendering**: Terminal-based text display with syntax highlighting and annotations
//! - **Command Processing**: Handling of edit and movement commands
//!
//! # Key Features
//!
//! - Unicode-aware text editing with proper grapheme cluster handling
//! - Efficient scrolling with viewport management
//! - Interactive search with highlighting and navigation
//! - File loading and saving operations
//! - Undo/redo support through the buffer system
//! - Syntax highlighting and text annotations
//!
//! # Example Usage
//!
//! ```rust
//! use hecto::editor::uicomponents::view::View;
//! use hecto::editor::Size;
//!
//! let mut view = View::default();
//! view.set_size(Size { width: 80, height: 24 });
//!
//! // Load a file
//! view.load("example.txt").expect("Failed to load file");
//!
//! // Handle user input
//! view.insert_char('H');
//! view.insert_char('i');
//! ```

use crate::prelude::*;

use super::{
    super::{
        DocumentStatus, Line, Terminal,
        command::{Edit, Move},
    },
    uicomponent::UIComponent,
};
mod buffer;
mod fileinfo;
mod highlighter;
mod searchdirection;
mod searchinfo;
use buffer::Buffer;
use fileinfo::FileInfo;
use highlighter::Highlighter;
use searchdirection::SearchDirection;
use searchinfo::SearchInfo;
use std::{cmp::min, io::Error, usize};
/// The main text editing view component.
///
/// `View` is the core component responsible for text editing functionality in the Hecto editor.
/// It manages document content, cursor positioning, scrolling, search operations, and renders
/// text to the terminal with proper Unicode handling and syntax highlighting.
///
/// # Fields
///
/// - `buffer`: The text buffer containing document content and file information
/// - `needs_redraw`: Flag indicating whether the view requires redrawing
/// - `size`: The current viewport dimensions (width and height)
/// - `text_location`: Current cursor position within the document
/// - `scroll_offset`: Current scroll position for viewport management
/// - `search_info`: Active search state, if any search operation is in progress
///
/// # Examples
///
/// ```rust
/// use hecto::editor::uicomponents::view::View;
/// use hecto::editor::Size;
///
/// let mut view = View::default();
/// view.set_size(Size { width: 80, height: 24 });
///
/// // Check if a file is loaded
/// if !view.is_file_loaded() {
///     println!("No file currently loaded");
/// }
/// ```
#[derive(Default)]
pub struct View {
    /// Text buffer containing document content and managing file operations
    buffer: Buffer,
    /// Flag indicating whether the view needs to be redrawn
    needs_redraw: bool,
    /// Current viewport dimensions
    size: Size,
    /// Current cursor position within the document
    text_location: Location,
    /// Current scroll offset for viewport positioning
    scroll_offset: Position,
    /// Active search information, if a search is in progress
    search_info: Option<SearchInfo>,
}
impl View {
    /// Gets the current document status information.
    ///
    /// Returns a `DocumentStatus` struct containing metadata about the current document,
    /// including total line count, current line index, file name, and modification status.
    /// This information is typically used by the status bar to display document information.
    ///
    /// # Returns
    ///
    /// A `DocumentStatus` struct with current document metadata
    ///
    /// # Examples
    ///
    /// ```rust
    /// let view = View::default();
    /// let status = view.get_status();
    /// println!("Current line: {}", status.current_line_idx + 1);
    /// println!("Total lines: {}", status.total_lines);
    /// ```
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_idx: self.text_location.line_idx,
            file_name: format!("{}", self.buffer.get_file_info()),
            is_modified: self.buffer.is_dirty(),
        }
    }

    /// Checks whether a file is currently loaded in the view.
    ///
    /// # Returns
    ///
    /// `true` if a file is loaded, `false` if the view contains an empty buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// assert!(!view.is_file_loaded());
    ///
    /// view.load("example.txt").unwrap();
    /// assert!(view.is_file_loaded());
    /// ```
    pub const fn is_file_loaded(&self) -> bool {
        self.buffer.is_file_loaded()
    }

    /// Enters search mode and saves the current view state.
    ///
    /// This method initializes search mode by creating a `SearchInfo` structure that
    /// preserves the current cursor location and scroll offset. This allows the user
    /// to return to the original position if the search is dismissed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// view.enter_search();
    /// // Search mode is now active
    /// ```
    pub fn enter_search(&mut self) {
        self.search_info = Some(SearchInfo {
            prev_location: self.text_location,
            prev_scroll_offset: self.scroll_offset,
            query: None,
        });
    }

    /// Exits search mode while staying at the current location.
    ///
    /// This method terminates the current search operation and clears search highlighting,
    /// but maintains the current cursor position. Use `dismiss_search()` to return to
    /// the original position before the search began.
    pub fn exit_search(&mut self) {
        self.search_info = None;
        self.set_needs_redraw(true);
    }

    /// Dismisses search mode and returns to the original position.
    ///
    /// This method not only exits search mode but also restores the cursor location
    /// and scroll offset to their positions before the search began. This is useful
    /// when the user wants to cancel a search operation entirely.
    pub fn dismiss_search(&mut self) {
        if let Some(search_info) = &self.search_info {
            self.text_location = search_info.prev_location;
            self.scroll_offset = search_info.prev_scroll_offset;
            self.scroll_text_location_into_view();
        }
        self.exit_search();
    }

    /// Performs a search for the given query string.
    ///
    /// Initiates a search operation using the specified query. The search begins from
    /// the current cursor position and proceeds in the default direction (forward).
    /// Empty queries are ignored.
    ///
    /// # Parameters
    ///
    /// * `query` - The text to search for
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// view.enter_search();
    /// view.search("hello");
    /// // Cursor moves to the first occurrence of "hello"
    /// ```
    pub fn search(&mut self, query: &str) {
        if let Some(search_info) = &mut self.search_info {
            search_info.query = Some(Line::from(query));
        }
        self.search_in_direction(self.text_location, SearchDirection::default());
    }
    /// Gets the current search query if search is active.
    ///
    /// Returns a reference to the current search query line, or `None` if no search
    /// is active. This method includes a debug assertion to ensure search info is
    /// properly initialized when called.
    ///
    /// # Returns
    ///
    /// An `Option<&Line>` containing the search query, or `None` if no search is active
    ///
    /// # Panics
    ///
    /// Panics in debug builds if called when search info is malformed
    fn get_search_query(&self) -> Option<&Line> {
        let query = self
            .search_info
            .as_ref()
            .and_then(|search_info| search_info.query.as_ref());
        debug_assert!(
            query.is_some(),
            "Attempting to search with malformed searchinfo present"
        );
        query
    }

    /// Performs a search in the specified direction from the given location.
    ///
    /// This is the core search method that handles both forward and backward searches.
    /// If a match is found, the cursor is moved to that location and the view is
    /// centered on the match.
    ///
    /// # Parameters
    ///
    /// * `from` - The starting location for the search
    /// * `direction` - The direction to search (forward or backward)
    fn search_in_direction(&mut self, from: Location, direction: SearchDirection) {
        if let Some(location) = self.get_search_query().and_then(|query| {
            if query.is_empty() {
                None
            } else if direction == SearchDirection::Forward {
                self.buffer.search_forward(query, from)
            } else {
                self.buffer.search_backward(query, from)
            }
        }) {
            self.text_location = location;
            self.center_text_location();
        }
        self.set_needs_redraw(true);
    }

    /// Searches for the next occurrence of the current query.
    ///
    /// Moves the cursor forward to find the next match of the current search query.
    /// The search starts from a position slightly ahead of the current cursor to
    /// avoid finding the same match repeatedly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// view.enter_search();
    /// view.search("hello");
    /// view.search_next(); // Find next "hello"
    /// ```
    pub fn search_next(&mut self) {
        let step_right = self
            .get_search_query()
            .map_or(1, |query| min(query.grapheme_count(), 1));
        let location = Location {
            line_idx: self.text_location.line_idx,
            grapheme_idx: self.text_location.grapheme_idx.saturating_add(step_right),
        };
        self.search_in_direction(location, SearchDirection::Forward);
    }

    /// Searches for the previous occurrence of the current query.
    ///
    /// Moves the cursor backward to find the previous match of the current search query.
    /// The search starts from the current cursor position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// view.enter_search();
    /// view.search("hello");
    /// view.search_prev(); // Find previous "hello"
    /// ```
    pub fn search_prev(&mut self) {
        self.search_in_direction(self.text_location, SearchDirection::Backward);
    }

    /// Loads a file into the view.
    ///
    /// Attempts to load the specified file into the view's buffer. If successful,
    /// the view is marked for redraw to display the new content.
    ///
    /// # Parameters
    ///
    /// * `file_name` - Path to the file to load
    ///
    /// # Returns
    ///
    /// `Ok(())` if the file was loaded successfully, or an `Error` if loading failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file does not exist
    /// - The file cannot be read due to permissions
    /// - The file contains invalid UTF-8 data
    /// - An I/O error occurs during reading
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// match view.load("example.txt") {
    ///     Ok(()) => println!("File loaded successfully"),
    ///     Err(e) => eprintln!("Failed to load file: {}", e),
    /// }
    /// ```
    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let buffer = Buffer::load(file_name)?;
        self.buffer = buffer;
        self.set_needs_redraw(true);
        Ok(())
    }

    /// Saves the current buffer to its associated file.
    ///
    /// Saves the buffer content to the file that was originally loaded. If the buffer
    /// was created without loading a file, this method will fail.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the file was saved successfully, or an `Error` if saving failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No file is associated with the buffer
    /// - The file cannot be written due to permissions
    /// - An I/O error occurs during writing
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// view.load("example.txt").unwrap();
    /// // Make some edits...
    /// view.save().expect("Failed to save file");
    /// ```
    pub fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()
    }

    /// Saves the current buffer to a new file.
    ///
    /// Saves the buffer content to the specified file, potentially creating a new file
    /// or overwriting an existing one.
    ///
    /// # Parameters
    ///
    /// * `file_name` - Path where the file should be saved
    ///
    /// # Returns
    ///
    /// `Ok(())` if the file was saved successfully, or an `Error` if saving failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created or written due to permissions
    /// - An I/O error occurs during writing
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut view = View::default();
    /// // Add some content...
    /// view.save_as("new_file.txt").expect("Failed to save file");
    /// ```
    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        self.buffer.save_as(file_name)
    }
    /// Handles edit commands that modify the document content.
    ///
    /// Processes various edit operations including character insertion, deletion,
    /// and newline insertion. After processing the command, the view is marked
    /// for redraw to reflect the changes.
    ///
    /// # Parameters
    ///
    /// * `command` - The edit command to execute
    ///
    /// # Edit Commands
    ///
    /// - `DeleteBackward`: Deletes the character before the cursor (backspace)
    /// - `Delete`: Deletes the character at the cursor position
    /// - `InsertNewline`: Inserts a newline character and moves cursor to next line
    /// - `Insert(char)`: Inserts the specified character at the cursor position
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::command::Edit;
    ///
    /// let mut view = View::default();
    /// view.handle_edit_command(Edit::Insert('H'));
    /// view.handle_edit_command(Edit::Insert('i'));
    /// view.handle_edit_command(Edit::InsertNewline);
    /// ```
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::DeleteBackward => self.delete_backward(),
            Edit::Delete => self.delete(),
            Edit::InsertNewline => self.insert_newline(),
            Edit::Insert(character) => self.insert_char(character),
        }
    }

    /// Handles movement commands that change the cursor position.
    ///
    /// Processes various cursor movement operations including directional movement,
    /// page navigation, and line boundary navigation. After processing the command,
    /// the view scrolls to ensure the cursor remains visible.
    ///
    /// # Parameters
    ///
    /// * `command` - The movement command to execute
    ///
    /// # Movement Commands
    ///
    /// - `Up/Down`: Move cursor up/down by one line
    /// - `PageUp/PageDown`: Move cursor up/down by viewport height
    /// - `Left/Right`: Move cursor left/right by one character
    /// - `StartOfLine/EndOfLine`: Move cursor to beginning/end of current line
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::command::Move;
    ///
    /// let mut view = View::default();
    /// view.handle_move_command(Move::Down);
    /// view.handle_move_command(Move::EndOfLine);
    /// ```
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
    /// Inserts a newline character at the current cursor position.
    ///
    /// This method inserts a newline into the buffer at the current cursor location,
    /// then moves the cursor to the beginning of the next line. The view is marked
    /// for redraw to reflect the changes.
    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.handle_move_command(Move::Right);
        self.set_needs_redraw(true);
    }

    /// Deletes the character before the cursor (backspace operation).
    ///
    /// If the cursor is not at the beginning of the document, this method moves
    /// the cursor left by one position and then deletes the character at that
    /// position. This effectively deletes the character before the original
    /// cursor position.
    fn delete_backward(&mut self) {
        if self.text_location.line_idx != 0 || self.text_location.grapheme_idx != 0 {
            self.handle_move_command(Move::Left);
            self.delete();
        }
    }

    /// Deletes the character at the current cursor position.
    ///
    /// Removes the character at the current cursor location from the buffer.
    /// The cursor position remains unchanged, but subsequent characters shift
    /// left to fill the gap. The view is marked for redraw.
    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.set_needs_redraw(true);
    }

    /// Inserts a character at the current cursor position.
    ///
    /// This method inserts the specified character into the buffer at the current
    /// cursor location. If the insertion increases the line length, the cursor
    /// is moved right to position after the inserted character. The view is
    /// marked for redraw.
    ///
    /// # Parameters
    ///
    /// * `character` - The Unicode character to insert
    fn insert_char(&mut self, character: char) {
        let old_len = self.buffer.grapheme_count(self.text_location.line_idx);
        self.buffer.insert_char(character, self.text_location);
        let new_len = self.buffer.grapheme_count(self.text_location.line_idx);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            self.handle_move_command(Move::Right);
        }
        self.set_needs_redraw(true);
    }
    /// Renders a single line of text to the terminal.
    ///
    /// This is a utility method that prints the specified text to the terminal
    /// at the given row position. It serves as a simple wrapper around the
    /// terminal's print functionality.
    ///
    /// # Parameters
    ///
    /// * `at` - The row position where the text should be rendered
    /// * `line_text` - The text content to render
    ///
    /// # Returns
    ///
    /// `Ok(())` if rendering succeeded, or an `Error` if terminal operations failed
    ///
    /// # Errors
    ///
    /// Returns an error if terminal output operations fail
    fn render_line(at: RowIdx, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }

    /// Builds a welcome message for display when no file is loaded.
    ///
    /// Creates a formatted welcome message that includes the editor name and version,
    /// centered within the specified width. If the width is too small to display
    /// the full message, it returns a simple tilde character.
    ///
    /// # Parameters
    ///
    /// * `width` - The available width for the welcome message
    ///
    /// # Returns
    ///
    /// A formatted string containing the welcome message
    ///
    /// # Examples
    ///
    /// ```rust
    /// let message = View::build_welcome_message(80);
    /// // Returns something like: "~        Hecto editor -- version 0.1.0        "
    /// ```
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);
        if remaining_width < len {
            return "~".to_string();
        }
        format!("{:1<}{:^remaining_width$}", "~", welcome_message)
    }
    /// Scrolls the view vertically to ensure the specified row is visible.
    ///
    /// Adjusts the vertical scroll offset to bring the target row into the current
    /// viewport. If the row is already visible, no scrolling occurs. The view is
    /// marked for redraw if scrolling takes place.
    ///
    /// # Parameters
    ///
    /// * `to` - The target row that should be visible
    ///
    /// # Behavior
    ///
    /// - If `to` is above the viewport, scrolls up to show it at the top
    /// - If `to` is below the viewport, scrolls down to show it at the bottom
    /// - If `to` is already visible, no action is taken
    fn scroll_vertically(&mut self, to: RowIdx) {
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

    /// Scrolls the view horizontally to ensure the specified column is visible.
    ///
    /// Adjusts the horizontal scroll offset to bring the target column into the
    /// current viewport. If the column is already visible, no scrolling occurs.
    /// The view is marked for redraw if scrolling takes place.
    ///
    /// # Parameters
    ///
    /// * `to` - The target column that should be visible
    ///
    /// # Behavior
    ///
    /// - If `to` is left of the viewport, scrolls left to show it at the left edge
    /// - If `to` is right of the viewport, scrolls right to show it at the right edge
    /// - If `to` is already visible, no action is taken
    fn scroll_horizontally(&mut self, to: ColIdx) {
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

    /// Centers the current cursor location in the viewport.
    ///
    /// Adjusts both horizontal and vertical scroll offsets to position the
    /// current cursor location at the center of the viewport. This is commonly
    /// used after search operations to ensure the found text is prominently
    /// displayed.
    fn center_text_location(&mut self) {
        let Size { height, width } = self.size;
        let Position { col, row } = self.text_location_to_position();
        let vertical_mid = height.div_ceil(2);
        let horizontal_mid = width.div_ceil(2);
        self.scroll_offset.row = row.saturating_sub(vertical_mid);
        self.scroll_offset.col = col.saturating_sub(horizontal_mid);
        self.set_needs_redraw(true);
    }

    /// Scrolls the view to ensure the current cursor location is visible.
    ///
    /// This method ensures that the current cursor position is within the visible
    /// viewport by adjusting scroll offsets as needed. Unlike `center_text_location`,
    /// this method only scrolls if necessary to bring the cursor into view.
    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }
    /// Gets the current cursor position relative to the viewport.
    ///
    /// Returns the position of the cursor within the current viewport, taking
    /// scroll offsets into account. This is used for rendering the cursor
    /// at the correct screen position.
    ///
    /// # Returns
    ///
    /// A `Position` representing the cursor location within the viewport
    ///
    /// # Examples
    ///
    /// ```rust
    /// let view = View::default();
    /// let cursor_pos = view.caret_position();
    /// println!("Cursor at row {}, col {}", cursor_pos.row, cursor_pos.col);
    /// ```
    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    /// Converts the current text location to an absolute screen position.
    ///
    /// Transforms the logical text location (line and grapheme indices) into
    /// absolute screen coordinates, taking into account line wrapping and
    /// Unicode grapheme cluster widths.
    ///
    /// # Returns
    ///
    /// A `Position` representing the absolute screen coordinates
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the current line index is invalid
    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_idx;
        debug_assert!(row.saturating_sub(1) <= self.buffer.height());
        let col = self
            .buffer
            .width_until(row, self.text_location.grapheme_idx);
        Position { col, row }
    }
    /// Moves the cursor up by the specified number of lines.
    ///
    /// Decreases the cursor's line index by the given step amount, ensuring
    /// it doesn't go below zero. After moving, the cursor is snapped to a
    /// valid grapheme position on the new line.
    ///
    /// # Parameters
    ///
    /// * `step` - Number of lines to move up
    fn move_up(&mut self, step: usize) {
        self.text_location.line_idx = self.text_location.line_idx.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    /// Moves the cursor down by the specified number of lines.
    ///
    /// Increases the cursor's line index by the given step amount. After moving,
    /// the cursor is snapped to valid line and grapheme positions to ensure
    /// it remains within document bounds.
    ///
    /// # Parameters
    ///
    /// * `step` - Number of lines to move down
    fn move_down(&mut self, step: usize) {
        self.text_location.line_idx = self.text_location.line_idx.saturating_add(step);
        self.snap_to_valid_line();
        self.snap_to_valid_grapheme();
    }

    /// Moves the cursor right by one grapheme cluster.
    ///
    /// Advances the cursor to the next grapheme position. If the cursor is at
    /// the end of a line, it wraps to the beginning of the next line. This
    /// method properly handles Unicode grapheme clusters for correct cursor
    /// movement.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self) {
        let grapheme_count = self.buffer.grapheme_count(self.text_location.line_idx);
        if self.text_location.grapheme_idx < grapheme_count {
            self.text_location.grapheme_idx += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    /// Moves the cursor left by one grapheme cluster.
    ///
    /// Moves the cursor to the previous grapheme position. If the cursor is at
    /// the beginning of a line, it wraps to the end of the previous line. This
    /// method properly handles Unicode grapheme clusters for correct cursor
    /// movement.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_idx > 0 {
            self.text_location.grapheme_idx -= 1;
        } else if self.text_location.line_idx > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    /// Moves the cursor to the beginning of the current line.
    ///
    /// Sets the cursor's grapheme index to zero, positioning it at the start
    /// of the current line.
    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_idx = 0;
    }

    /// Moves the cursor to the end of the current line.
    ///
    /// Sets the cursor's grapheme index to the length of the current line,
    /// positioning it after the last character on the line.
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_idx = self.buffer.grapheme_count(self.text_location.line_idx);
    }
    /// Ensures the cursor's grapheme index is valid for the current line.
    ///
    /// Adjusts the cursor's grapheme index to ensure it doesn't exceed the
    /// length of the current line. If the current line is shorter than the
    /// cursor position, the cursor is moved to the end of the line.
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_idx = min(
            self.text_location.grapheme_idx,
            self.buffer.grapheme_count(self.text_location.line_idx),
        );
    }

    /// Ensures the cursor's line index is valid within the document.
    ///
    /// Adjusts the cursor's line index to ensure it doesn't exceed the
    /// number of lines in the document. If the cursor is beyond the last
    /// line, it's moved to the end of the document.
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_idx = min(self.text_location.line_idx, self.buffer.height());
    }
}
/// Implementation of the `UIComponent` trait for the `View`.
///
/// This implementation provides the standard UI component interface for the view,
/// including redraw management, size handling, and rendering functionality.
impl UIComponent for View {
    /// Sets whether the view needs to be redrawn.
    ///
    /// # Parameters
    ///
    /// * `value` - `true` if the view needs redrawing, `false` otherwise
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    /// Checks whether the view needs to be redrawn.
    ///
    /// # Returns
    ///
    /// `true` if the view needs redrawing, `false` otherwise
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Sets the size of the view viewport.
    ///
    /// Updates the view's size and ensures the current cursor position
    /// remains visible within the new viewport dimensions.
    ///
    /// # Parameters
    ///
    /// * `size` - The new viewport dimensions
    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    /// Renders the view content to the terminal.
    ///
    /// This method handles the complete rendering process for the view, including:
    /// - Displaying document lines within the viewport
    /// - Showing search highlights when search is active
    /// - Rendering the welcome message for empty documents
    /// - Filling empty areas with tilde characters
    ///
    /// # Parameters
    ///
    /// * `origin_row` - The starting row position for rendering
    ///
    /// # Returns
    ///
    /// `Ok(())` if rendering succeeded, or an `Error` if terminal operations failed
    ///
    /// # Errors
    ///
    /// Returns an error if any terminal output operations fail during rendering
    ///
    /// # Rendering Behavior
    ///
    /// - Document lines are rendered with proper Unicode handling
    /// - Search matches are highlighted when search is active
    /// - The welcome message is shown in the top third of empty documents
    /// - Empty lines are filled with tilde characters (similar to Vi/Vim)
    fn draw(&mut self, origin_row: RowIdx) -> Result<(), Error> {
        let Size { height, width } = self.size;
        let end_y = origin_row.saturating_add(height);
        let top_third = height.div_ceil(3);
        let scroll_top = self.scroll_offset.row;

        let query = self
            .search_info
            .as_ref()
            .and_then(|search_info| search_info.query.as_deref());
        let selected_match = query.is_some().then_some(self.text_location);
        let mut highlighter = Highlighter::new(query, selected_match);

        for current_row in 0..end_y {
            self.buffer.highlight(current_row, &mut highlighter);
        }

        for current_row in origin_row..end_y {
            let line_idx = current_row
                .saturating_sub(origin_row)
                .saturating_add(scroll_top);
            let left = self.scroll_offset.col;
            let right = self.scroll_offset.col.saturating_add(width);
            if let Some(annotated_string) =
                self.buffer
                    .get_highlighted_substring(line_idx, left..right, &highlighter)
            {
                Terminal::print_annotated_row(current_row, &annotated_string)?;
            } else if current_row == top_third && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        Ok(())
    }
}
