/// Represents the size of the terminal.
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct Size {
    /// The height of the terminal.
    pub height: usize,
    /// The width of the terminal.
    pub width: usize,
}

impl Size {
    /// Creates a new `Size` instance.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the terminal.
    /// * `width` - The width of the terminal.
    ///
    /// # Returns
    ///
    /// A new `Size` instance.
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    /// Returns the area of the terminal.
    ///
    /// # Returns
    ///
    /// The area of the terminal.
    pub fn area(&self) -> usize {
        self.height * self.width
    }

    /// Checks if the terminal size is zero.
    ///
    /// # Returns
    ///
    /// `true` if the terminal size is zero, `false` otherwise.
    pub fn is_zero(&self) -> bool {
        self.height == 0 || self.width == 0
    }
}
