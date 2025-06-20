//! # Move Command Module
//!
//! This module defines cursor movement commands that navigate within the document
//! without modifying the text content. Move commands handle various navigation
//! operations including arrow keys, page navigation, and line boundaries.

use crossterm::event::{
    KeyCode::{Down, End, Home, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyModifiers,
};

/// Represents cursor movement operations
///
/// Move commands change the cursor position within the document without
/// modifying the text content. These operations are used for navigation
/// and positioning before performing edit operations.
#[derive(Clone, Copy)]
pub enum Move {
    /// Move cursor up one page
    PageUp,
    /// Move cursor down one page
    PageDown,
    /// Move cursor to the start of the current line
    StartOfLine,
    /// Move cursor to the end of the current line
    EndOfLine,
    /// Move cursor left one character position
    Left,
    /// Move cursor right one character position
    Right,
    /// Move cursor up one line
    Up,
    /// Move cursor down one line
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    /// Attempts to convert a keyboard event into a move command
    ///
    /// This method parses keyboard input to determine what cursor movement
    /// operation should be performed. It only accepts movement keys without
    /// modifier keys (plain key presses).
    ///
    /// # Arguments
    ///
    /// * `value` - The keyboard event to convert
    ///
    /// # Returns
    ///
    /// Returns `Ok(Move)` if the key event represents a movement operation,
    /// or `Err(String)` if the key combination is not supported for movement.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
    ///
    /// let left_event = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
    /// let move_cmd = Move::try_from(left_event).unwrap();
    /// // This would be Move::Left
    ///
    /// let home_event = KeyEvent::new(KeyCode::Home, KeyModifiers::NONE);
    /// let home_cmd = Move::try_from(home_event).unwrap();
    /// // This would be Move::StartOfLine
    /// ```
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;

        // Only accept movement keys without modifiers
        if modifiers == KeyModifiers::NONE {
            match code {
                PageUp => Ok(Self::PageUp),
                PageDown => Ok(Self::PageDown),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
