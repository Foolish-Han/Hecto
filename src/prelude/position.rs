//! # Position Module
//!
//! This module defines types and utilities for representing positions within the text editor.
//! It provides a simple coordinate system using row and column indices.

use super::{ColIdx, RowIdx};
/// Represents a 2D position in the text editor
///
/// A Position consists of a column and row coordinate, both using 0-based indexing.
/// This is used throughout the editor to track cursor positions, scroll offsets,
/// and other location-based operations.
///
/// # Examples
///
/// ```
/// let cursor_pos = Position { col: 10, row: 5 };
/// let origin = Position::default(); // { col: 0, row: 0 }
/// ```
#[derive(Clone, Copy, Default)]
pub struct Position {
    /// Column coordinate (horizontal position)
    pub col: ColIdx,
    /// Row coordinate (vertical position)  
    pub row: RowIdx,
}

impl Position {
    /// Performs saturating subtraction on both coordinates
    ///
    /// This method subtracts another position from this one, using saturating
    /// arithmetic to prevent underflow. If the subtraction would result in a
    /// negative value, it returns 0 instead.
    ///
    /// # Arguments
    ///
    /// * `other` - The position to subtract from this position
    ///
    /// # Returns
    ///
    /// A new Position with the result of the saturating subtraction
    ///
    /// # Examples
    ///
    /// ```
    /// let pos1 = Position { col: 10, row: 5 };
    /// let pos2 = Position { col: 3, row: 2 };
    /// let result = pos1.saturating_sub(pos2);
    /// // result == Position { col: 7, row: 3 }
    ///
    /// let pos3 = Position { col: 2, row: 1 };
    /// let pos4 = Position { col: 5, row: 3 };
    /// let result2 = pos3.saturating_sub(pos4);
    /// // result2 == Position { col: 0, row: 0 } (saturated)
    /// ```
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            col: self.col.saturating_sub(other.col),
            row: self.row.saturating_sub(other.row),
        }
    }
}
