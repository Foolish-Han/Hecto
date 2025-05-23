use super::{FileInfo, Line, Location};
use std::{
    fs::{File, read_to_string},
    io::{Error, Write},
};

/// Represents a text buffer.
#[derive(Default)]
pub struct Buffer {
    /// The lines of text in the buffer.
    pub lines: Vec<Line>,
    /// Information about the file associated with the buffer.
    pub file_info: FileInfo,
    /// Indicates whether the buffer has been modified.
    pub dirty: bool,
}

impl Buffer {
    /// Loads a file into the buffer.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to load.
    ///
    /// # Returns
    ///
    /// A result containing the loaded buffer or an error.
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

    pub fn search(&self, query: &str, from: Location) -> Option<Location> {
        for (line_idx, line) in self.lines.iter().enumerate().skip(from.line_idx) {
            let from_grapheme_idx = if line_idx == from.line_idx {
                from.grapheme_idx
            } else {
                0
            };
            if let Some(grapheme_idx) = line.search(query, from_grapheme_idx) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }
        for (line_idx, line) in self.lines.iter().enumerate().take(from.line_idx) {
            if let Some(grapheme_idx) = line.search(query, 0) {
                return Some(Location {
                    grapheme_idx,
                    line_idx,
                });
            }
        }
        None
    }

    fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(file_path) = &file_info.get_path() {
            let mut file = File::create(file_path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }

    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(file_name);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.dirty = false;
        Ok(())
    }

    /// Saves the buffer to a file.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn save(&mut self) -> Result<(), Error> {
        self.save_to_file(&self.file_info)?;
        self.dirty = false;
        Ok(())
    }

    /// Checks if the buffer is empty.
    ///
    /// # Returns
    ///
    /// `true` if the buffer is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub const fn is_file_loaded(&self) -> bool {
        self.file_info.has_path()
    }

    /// Returns the height of the buffer (number of lines).
    ///
    /// # Returns
    ///
    /// The height of the buffer.
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Inserts a character at the specified location in the buffer.
    ///
    /// # Arguments
    ///
    /// * `character` - The character to insert.
    /// * `at` - The location to insert the character.
    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_idx > self.height() {
            return;
        }

        if at.line_idx == self.height() {
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx) {
            line.insert_char(character, at.grapheme_idx);
            self.dirty = true;
        }
    }

    /// Deletes a character at the specified location in the buffer.
    ///
    /// # Arguments
    ///
    /// * `at` - The location to delete the character.
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_idx) {
            if at.grapheme_idx >= line.grapheme_count()
                && self.height() > at.line_idx.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_idx.saturating_add(1));

                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_idx < line.grapheme_count() {
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].delete(at.grapheme_idx);
                self.dirty = true;
            }
        }
    }

    /// Inserts a newline at the specified location in the buffer.
    ///
    /// # Arguments
    ///
    /// * `at` - The location to insert the newline.
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
