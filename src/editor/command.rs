use crossterm::event::{
    Event,
    KeyCode::{
        self, Backspace, Char, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab,
        Up,
    },
    KeyEvent, KeyModifiers,
};
use std::{convert::TryFrom, usize};

use super::Size;

/// Represents movement commands in the editor.
#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    StartOfLine,
    EndOfLine,
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    /// Converts a `KeyEvent` into a `Move` command.
    ///
    /// # Arguments
    ///
    /// * `value` - The `KeyEvent` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Move` command or an error message.
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;
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
                "Unsupported code: {code:?} or modifiers: {modifiers:?}"
            ))
        }
    }
}

/// Represents editing commands in the editor.
#[derive(Clone, Copy)]
pub enum Edit {
    Insert(char),
    InsertNewline,
    Delete,
    DeleteBackwards,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    /// Converts a `KeyEvent` into an `Edit` command.
    ///
    /// # Arguments
    ///
    /// * `value` - The `KeyEvent` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Edit` command or an error message.
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match (value.code, value.modifiers) {
            (Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter, KeyModifiers::NONE) => Ok(Self::InsertNewline),
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            (Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackwards),
            _ => Err(format!(
                "Unsupported code: {:?} with modifiers: {:?}",
                value.code, value.modifiers
            )),
        }
    }
}

/// Represents system commands in the editor.
#[derive(Clone, Copy)]
pub enum System {
    Resize(Size),
    Save,
    Quit,
    Dismiss,
    Search,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    /// Converts a `KeyEvent` into a `System` command.
    ///
    /// # Arguments
    ///
    /// * `value` - The `KeyEvent` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `System` command or an error message.
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('q') => Ok(Self::Quit),
                Char('s') => Ok(Self::Save),
                Char('f') => Ok(Self::Search),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}

/// Represents all possible commands in the editor.
#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    /// Converts an `Event` into a `Command`.
    ///
    /// # Arguments
    ///
    /// * `value` - The `Event` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Command` or an error message.
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
