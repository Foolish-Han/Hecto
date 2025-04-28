use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

/// Represents the direction of movement in the editor.
#[derive(Clone, Copy)]
pub enum Direction {
    /// Move one page up.
    PageUp,
    /// Move one page down.
    PageDown,
    /// Move to the beginning of the line.
    Home,
    /// Move to the end of the line.
    End,
    /// Move one character to the left.
    Left,
    /// Move one character to the right.
    Right,
    /// Move one line up.
    Up,
    /// Move one line down.
    Down,
}

/// Represents a command that can be executed by the editor.
#[derive(Clone, Copy)]
pub enum EditorCommand {
    /// Move the caret in the specified direction.
    Move(Direction),
    /// Resize the editor to the specified size.
    Resize(Size),
    /// Quit the editor.
    Quit,
    /// Insert the specified character at the current caret position.
    Insert(char),
    /// Delete the character before the caret.
    Backspace,
    /// Delete the character at the caret position.
    Delete,
    /// Insert a newline at the caret position.
    Enter,
    /// Save the current document.
    Save,
}

/// Attempts to convert an `Event` into an `EditorCommand`.
///
/// # Errors
///
/// Returns an error string if the event cannot be converted into a valid command.
#[allow(clippy::as_conversions)]
impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(Self::Insert(character))
                }
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => {
                let height = height_u16 as usize;
                let width = width_u16 as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
