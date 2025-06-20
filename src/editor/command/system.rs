use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

use super::super::Size;

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
