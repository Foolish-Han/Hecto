pub type Row = usize;
pub type Col = usize;

/// Represents a position in the terminal.
#[derive(Clone, Copy, Default)]
pub struct Position {
    /// The column position.
    pub col: Col,
    /// The row position.
    pub row: Row,
}

impl Position {
    /// Subtracts another position from this position, saturating at zero.
    ///
    /// # Arguments
    ///
    /// * `other` - The position to subtract.
    ///
    /// # Returns
    ///
    /// A new position representing the result of the subtraction.
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            col: self.col.saturating_sub(other.col),
            row: self.row.saturating_sub(other.row),
        }
    }
}
