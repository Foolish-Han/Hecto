//! # System Command Module
//!
//! This module defines system-level commands that control the editor's overall
//! behavior and state. System commands handle operations like saving, quitting,
//! searching, and responding to terminal events.

use crate::prelude::*;

use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

/// Represents system-level editor operations
///
/// System commands control the overall behavior of the editor rather than
/// directly modifying text content. These include file operations, search
/// functionality, and application lifecycle management.
#[derive(Clone, Copy)]
pub enum System {
    /// Terminal resize event with new dimensions
    Resize(Size),
    /// Save the current document
    Save,
    /// Quit the editor application
    Quit,
    /// Dismiss the current prompt or operation (usually Escape key)
    Dismiss,
    /// Enter search mode
    Search,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    /// Attempts to convert a keyboard event into a system command
    ///
    /// This method parses keyboard input to determine what system operation
    /// should be performed. It handles:
    /// - Control key combinations (Ctrl+S, Ctrl+Q, Ctrl+F)
    /// - Escape key for dismissing prompts
    ///
    /// # Arguments
    ///
    /// * `value` - The keyboard event to convert
    ///
    /// # Returns
    ///
    /// Returns `Ok(System)` if the key event represents a system operation,
    /// or `Err(String)` if the key combination is not supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
    ///
    /// let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
    /// let system_cmd = System::try_from(quit_event).unwrap();
    /// // This would be System::Quit
    ///
    /// let escape_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    /// let dismiss_cmd = System::try_from(escape_event).unwrap();
    /// // This would be System::Dismiss
    /// ```
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;

        if modifiers == KeyModifiers::CONTROL {
            // Handle Control key combinations
            match code {
                Char('q') => Ok(Self::Quit),
                Char('s') => Ok(Self::Save),
                Char('f') => Ok(Self::Search),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            // Handle Escape key
            Ok(Self::Dismiss)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
