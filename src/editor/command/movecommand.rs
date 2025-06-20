use crossterm::event::{
    KeyCode::{Down, End, Home, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyModifiers,
};
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
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
