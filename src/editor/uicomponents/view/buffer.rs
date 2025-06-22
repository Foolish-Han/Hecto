//! Text buffer management for the Hecto text editor.
//!
//! This module provides the `Buffer` struct, which manages the actual text content
//! of documents being edited. The buffer handles file I/O operations, text manipulation,
//! search functionality, and tracks modification state.
//!
//! # Key Features
//!
//! - **File Operations**: Loading from and saving to files
//! - **Text Manipulation**: Character insertion, deletion, and line operations
//! - **Search Functionality**: Forward and backward text search with wrapping
//! - **State Tracking**: Modification status and file association
//! - **Unicode Support**: Proper handling of Unicode grapheme clusters
//!
//! # Examples
//!
//! ```rust
//! use hecto::editor::uicomponents::view::buffer::Buffer;
//! use hecto::editor::uicomponents::view::Location;
//!
//! // Load a file
//! let mut buffer = Buffer::load("example.txt").expect("Failed to load file");
//!
//! // Insert text
//! buffer.insert_char('H', Location { line_idx: 0, grapheme_idx: 0 });
//!
//! // Save changes
//! buffer.save().expect("Failed to save file");
//! ```

use crate::{editor::annotatedstring::AnnotatedString, prelude::*};

use std::{
    fs::{File, read_to_string},
    io::{Error, Write},
    ops::Range,
};

use super::{FileInfo, Highlighter, Line};
/// A text buffer that manages document content and file operations.
///
/// The `Buffer` struct represents the core text storage for a document, providing
/// functionality for text manipulation, file I/O, search operations, and state tracking.
/// It maintains a collection of lines and associated file information.
///
/// # Fields
///
/// - `lines`: Vector of `Line` objects containing the document content
/// - `file_info`: Information about the associated file (path, name, etc.)
/// - `dirty`: Flag indicating whether the buffer has unsaved changes
///
/// # Examples
///
/// ```rust
/// use hecto::editor::uicomponents::view::buffer::Buffer;
///
/// // Create an empty buffer
/// let mut buffer = Buffer::default();
///
/// // Load a file
/// let mut buffer = Buffer::load("example.txt").expect("Failed to load file");
///
/// // Check if buffer has unsaved changes
/// if buffer.dirty {
///     println!("Buffer has unsaved changes");
/// }
/// ```
#[derive(Default)]
pub struct Buffer {
    /// The lines of text that make up the document
    lines: Vec<Line>,
    /// Information about the file associated with this buffer
    file_info: FileInfo,
    /// Whether the buffer has been modified since the last save
    dirty: bool,
}
impl Buffer {
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub const fn get_file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn grapheme_count(&self, idx: LineIdx) -> GraphemeIdx {
        self.lines.get(idx).map_or(0, |line| line.grapheme_count())
    }

    pub fn width_until(&self, idx: LineIdx, until: GraphemeIdx) -> GraphemeIdx {
        self.lines
            .get(idx)
            .map_or(0, |line| line.width_until(until))
    }
    pub fn get_highlighted_substring(
        &self,
        line_idx: LineIdx,
        range: Range<GraphemeIdx>,
        highlighter: &Highlighter,
    ) -> Option<AnnotatedString> {
        self.lines.get(line_idx).map(|line| {
            line.get_annotated_visible_substr(range, highlighter.get_annotations(line_idx))
        })
    }
    pub fn highlight(&self, idx: LineIdx, highlighter: &mut Highlighter) {
        if let Some(line) = self.lines.get(idx) {
            highlighter.highlight(idx, line);
        }
    }
    /// Loads a text file into a new buffer.
    ///
    /// Reads the specified file from disk and creates a new buffer containing
    /// its contents. Each line of the file becomes a `Line` object in the buffer.
    /// The buffer is initially marked as not dirty (no unsaved changes).
    ///
    /// # Parameters
    ///
    /// * `file_name` - Path to the file to load
    ///
    /// # Returns
    ///
    /// `Ok(Buffer)` containing the file contents, or an `Error` if loading failed
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
    /// use hecto::editor::uicomponents::view::buffer::Buffer;
    ///
    /// match Buffer::load("example.txt") {
    ///     Ok(buffer) => println!("Loaded {} lines", buffer.height()),
    ///     Err(e) => eprintln!("Failed to load file: {}", e),
    /// }
    /// ```
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self {
            lines,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }

    /// Searches forward for the specified query string.
    ///
    /// Performs a forward search starting from the given location, wrapping around
    /// to the beginning of the document if necessary. The search continues until
    /// a match is found or all lines have been searched.
    ///
    /// # Parameters
    ///
    /// * `query` - The text to search for
    /// * `from` - The starting location for the search
    ///
    /// # Returns
    ///
    /// `Some(Location)` of the first match found, or `None` if no match exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::{Buffer, Location};
    ///
    /// let buffer = Buffer::load("example.txt").unwrap();
    /// let start = Location { line_idx: 0, grapheme_idx: 0 };
    ///
    /// if let Some(location) = buffer.search_forward("hello", start) {
    ///     println!("Found 'hello' at line {}, column {}",
    ///              location.line_idx, location.grapheme_idx);
    /// }
    /// ```
    pub fn search_forward(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }
        let mut is_first = true;
        for (line_idx, line) in self
            .lines
            .iter()
            .enumerate()
            .cycle()
            .skip(from.line_idx)
            .take(self.lines.len().saturating_add(1))
        {
            let from_grapheme_idx = if is_first {
                is_first = false;
                from.grapheme_idx
            } else {
                0
            };
            if let Some(grapheme_idx) = line.search_forward(query, from_grapheme_idx) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }
        None
    }

    /// Searches backward for the specified query string.
    ///
    /// Performs a backward search starting from the given location, wrapping around
    /// to the end of the document if necessary. The search continues until a match
    /// is found or all lines have been searched.
    ///
    /// # Parameters
    ///
    /// * `query` - The text to search for
    /// * `from` - The starting location for the search
    ///
    /// # Returns
    ///
    /// `Some(Location)` of the first match found, or `None` if no match exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::{Buffer, Location};
    ///
    /// let buffer = Buffer::load("example.txt").unwrap();
    /// let start = Location { line_idx: 10, grapheme_idx: 5 };
    ///
    /// if let Some(location) = buffer.search_backward("hello", start) {
    ///     println!("Found 'hello' at line {}, column {}",
    ///              location.line_idx, location.grapheme_idx);
    /// }
    /// ```
    pub fn search_backward(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }
        let mut is_first = true;
        for (line_idx, line) in self
            .lines
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(
                self.lines
                    .len()
                    .saturating_sub(from.line_idx)
                    .saturating_sub(1),
            )
            .take(self.lines.len().saturating_add(1))
        {
            let from_grapheme_idx = if is_first {
                is_first = false;
                from.grapheme_idx
            } else {
                line.grapheme_count()
            };
            if let Some(grapheme_idx) = line.search_backward(query, from_grapheme_idx) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }
        None
    }
    /// Saves the buffer content to the specified file.
    ///
    /// This is an internal method that handles the actual file writing operation.
    /// It writes each line of the buffer to the file, adding newline characters
    /// between lines.
    ///
    /// # Parameters
    ///
    /// * `file_info` - Information about the target file
    ///
    /// # Returns
    ///
    /// `Ok(())` if saving succeeded, or an `Error` if the operation failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created or opened for writing
    /// - Write operations fail due to disk space or permissions
    /// - No file path is associated with the file info
    ///
    /// # Panics
    ///
    /// Panics in debug builds if no file path is present in the file info
    fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(file_path) = &file_info.get_path() {
            let mut file = File::create(file_path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Attempting to save with no file path present");
            }
        }
        Ok(())
    }

    /// Saves the buffer to a new file with the specified name.
    ///
    /// Creates a new file with the given name and writes the buffer content to it.
    /// After successful saving, the buffer's file info is updated to point to the
    /// new file, and the dirty flag is cleared.
    ///
    /// # Parameters
    ///
    /// * `file_name` - Path where the file should be saved
    ///
    /// # Returns
    ///
    /// `Ok(())` if saving succeeded, or an `Error` if the operation failed
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::default();
    /// // Add some content...
    /// buffer.save_as("new_file.txt").expect("Failed to save file");
    /// assert!(!buffer.dirty); // Should be clean after saving
    /// ```
    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(file_name);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.dirty = false;
        Ok(())
    }

    /// Saves the buffer to its currently associated file.
    ///
    /// Writes the buffer content to the file that was originally loaded or
    /// previously saved. After successful saving, the dirty flag is cleared.
    ///
    /// # Returns
    ///
    /// `Ok(())` if saving succeeded, or an `Error` if the operation failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No file is associated with the buffer
    /// - The file cannot be written
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::load("example.txt").unwrap();
    /// // Make some modifications...
    /// buffer.save().expect("Failed to save file");
    /// assert!(!buffer.dirty); // Should be clean after saving
    /// ```
    pub fn save(&mut self) -> Result<(), Error> {
        self.save_to_file(&self.file_info)?;
        self.dirty = false;
        Ok(())
    }
    /// Checks if the buffer is empty (contains no lines).
    ///
    /// # Returns
    ///
    /// `true` if the buffer contains no lines, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::buffer::Buffer;
    ///
    /// let buffer = Buffer::default();
    /// assert!(buffer.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Checks if a file is currently loaded in the buffer.
    ///
    /// # Returns
    ///
    /// `true` if the buffer is associated with a file, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::default();
    /// assert!(!buffer.is_file_loaded());
    ///
    /// buffer = Buffer::load("example.txt").unwrap();
    /// assert!(buffer.is_file_loaded());
    /// ```
    pub const fn is_file_loaded(&self) -> bool {
        self.file_info.has_path()
    }

    /// Gets the number of lines in the buffer.
    ///
    /// # Returns
    ///
    /// The total number of lines in the buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::buffer::Buffer;
    ///
    /// let buffer = Buffer::load("example.txt").unwrap();
    /// println!("Buffer contains {} lines", buffer.height());
    /// ```
    pub fn height(&self) -> LineIdx {
        self.lines.len()
    }
    /// Inserts a character at the specified location.
    ///
    /// Inserts the given character into the buffer at the specified location.
    /// If the location is beyond the end of the buffer, a new line is created.
    /// The buffer is marked as dirty after the operation.
    ///
    /// # Parameters
    ///
    /// * `character` - The Unicode character to insert
    /// * `at` - The location where the character should be inserted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::{Buffer, Location};
    ///
    /// let mut buffer = Buffer::default();
    /// let location = Location { line_idx: 0, grapheme_idx: 0 };
    /// buffer.insert_char('H', location);
    /// assert!(buffer.dirty);
    /// ```
    pub fn insert_char(&mut self, character: char, at: Location) {
        debug_assert!(at.line_idx <= self.height());
        if at.line_idx == self.height() {
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx) {
            line.insert_char(character, at.grapheme_idx);
            self.dirty = true;
        }
    }

    /// Deletes a character at the specified location.
    ///
    /// Removes the character at the given location. If the location is at the end
    /// of a line and there's a next line, the next line is merged with the current
    /// line. The buffer is marked as dirty after the operation.
    ///
    /// # Parameters
    ///
    /// * `at` - The location where the character should be deleted
    ///
    /// # Behavior
    ///
    /// - If at end of line and next line exists: merges lines
    /// - If within a line: deletes the character at that position
    /// - If location is invalid: no operation is performed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::{Buffer, Location};
    ///
    /// let mut buffer = Buffer::load("example.txt").unwrap();
    /// let location = Location { line_idx: 0, grapheme_idx: 5 };
    /// buffer.delete(location);
    /// assert!(buffer.dirty);
    /// ```
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_idx) {
            if at.grapheme_idx >= line.grapheme_count()
                && self.height() > at.line_idx.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_idx.saturating_add(1));
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_idx < line.grapheme_count() {
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].delete(at.grapheme_idx);
                self.dirty = true;
            }
        }
    }

    /// Inserts a newline at the specified location.
    ///
    /// Splits the line at the given location, creating a new line with the
    /// content that was after the split point. The buffer is marked as dirty
    /// after the operation.
    ///
    /// # Parameters
    ///
    /// * `at` - The location where the newline should be inserted
    ///
    /// # Behavior
    ///
    /// - If at end of buffer: adds a new empty line
    /// - If within a line: splits the line at the specified position
    /// - Content after the split point moves to the new line
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::{Buffer, Location};
    ///
    /// let mut buffer = Buffer::load("example.txt").unwrap();
    /// let location = Location { line_idx: 0, grapheme_idx: 10 };
    /// buffer.insert_newline(location);
    /// assert!(buffer.dirty);
    /// ```
    pub fn insert_newline(&mut self, at: Location) {
        if at.line_idx == self.height() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx) {
            let newline = line.split(at.grapheme_idx);
            self.lines.insert(at.line_idx.saturating_add(1), newline);
            self.dirty = true;
        }
    }
}
