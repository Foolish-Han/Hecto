/// Represents the size of the terminal.
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct Size {
    /// The height of the terminal.
    pub height: usize,
    /// The width of the terminal.
    pub width: usize,
}
