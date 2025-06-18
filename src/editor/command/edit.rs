use crossterm::event::{KeyCode::{Backspace, Char, Delete, Enter, Tab}, KeyEvent, KeyModifiers};

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
