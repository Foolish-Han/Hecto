//! # Command Module
//!
//! This module defines the command system for the Hecto text editor. It provides
//! a hierarchical command structure that converts terminal events into high-level
//! editor operations.
//!
//! ## Command Hierarchy
//!
//! Commands are organized into three main categories:
//! - **Move**: Cursor movement operations (arrow keys, page up/down, home/end)
//! - **Edit**: Text editing operations (insert, delete, newline)
//! - **System**: System-level operations (save, quit, search, resize)
//!
//! ## Event Processing
//!
//! Terminal events are converted to commands through the `TryFrom<Event>` implementation,
//! which attempts to parse keyboard events and resize events into appropriate command types.

use std::{convert::TryFrom, usize};

use crossterm::event::Event;

use super::Size;

mod edit;
mod movecommand;
mod system;

pub use edit::Edit;
pub use movecommand::Move;
pub use system::System;

/// Represents a high-level editor command
///
/// Commands are the primary way that user input is processed within the editor.
/// Each command represents a specific action that can be performed, whether it's
/// moving the cursor, editing text, or performing system operations.
#[derive(Clone, Copy)]
pub enum Command {
    /// Cursor movement command
    Move(Move),
    /// Text editing command
    Edit(Edit),
    /// System operation command
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    /// Attempts to convert a terminal event into an editor command
    ///
    /// This method tries to parse the event as each command type in order:
    /// 1. First attempts to parse as an Edit command
    /// 2. Then attempts to parse as a Move command  
    /// 3. Finally attempts to parse as a System command
    /// 4. Resize events are handled specially and converted to System::Resize
    ///
    /// # Arguments
    ///
    /// * `value` - The terminal event to convert
    ///
    /// # Returns
    ///
    /// Returns `Ok(Command)` if the event can be converted to a command,
    /// or `Err(String)` with an error description if the event is not supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use crossterm::event::{Event, KeyEvent, KeyCode, KeyModifiers};
    ///
    /// let key_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    /// let event = Event::Key(key_event);
    /// let command = Command::try_from(event).unwrap();
    /// // This would be parsed as Command::Edit(Edit::Insert('a'))
    /// ```
    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {:?}", key_event)),
            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            }))),
            _ => Err(format!("Event not supported: {:?}", value)),
        }
    }
}
