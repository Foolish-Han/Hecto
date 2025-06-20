//! # Edit Command Module
//!
//! This module defines editing commands that modify the text content of the document.
//! Edit commands handle character insertion, deletion, and line breaks.

use crossterm::event::{
    KeyCode::{Backspace, Char, Delete, Enter, Tab},
    KeyEvent, KeyModifiers,
};

/// Represents text editing operations
///
/// Edit commands modify the content of the document by inserting or deleting
/// characters and managing line breaks. These commands directly affect the
/// text buffer and mark the document as modified.
#[derive(Clone, Copy)]
pub enum Edit {
    /// Insert a character at the current cursor position
    Insert(char),
    /// Insert a newline character (line break)
    InsertNewline,
    /// Delete the character at the current cursor position (Delete key)
    Delete,
    /// Delete the character before the current cursor position (Backspace key)
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    /// Attempts to convert a keyboard event into an edit command
    ///
    /// This method parses keyboard input to determine what editing operation
    /// should be performed. It handles:
    /// - Regular character input (with or without Shift modifier)
    /// - Tab key (converted to tab character)
    /// - Enter key (converted to newline)
    /// - Delete and Backspace keys
    ///
    /// # Arguments
    ///
    /// * `value` - The keyboard event to convert
    ///
    /// # Returns
    ///
    /// Returns `Ok(Edit)` if the key event represents an edit operation,
    /// or `Err(String)` if the key combination is not supported for editing.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
    ///
    /// let char_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    /// let edit_cmd = Edit::try_from(char_event).unwrap();
    /// // This would be Edit::Insert('a')
    ///
    /// let enter_event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    /// let newline_cmd = Edit::try_from(enter_event).unwrap();
    /// // This would be Edit::InsertNewline
    /// ```
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match (value.code, value.modifiers) {
            // Regular character input (with or without Shift)
            (Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            },
            // Tab key - insert tab character
            (Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            // Enter key - insert newline
            (Enter, KeyModifiers::NONE) => Ok(Self::InsertNewline),
            // Delete key - delete character at cursor
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            // Backspace key - delete character before cursor
            (Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            // Unsupported key combination
            _ => Err(format!(
                "Unsupported key code {:?} with modifier {:?}",
                value.code, value.modifiers
            )),
        }
    }
}
