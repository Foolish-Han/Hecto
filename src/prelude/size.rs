//! # Size Module
//!
//! This module defines the Size structure used throughout the editor to represent
//! rectangular dimensions such as terminal size, component sizes, and viewport dimensions.

/// Represents a 2D size with width and height dimensions
///
/// Size is used throughout the editor to represent the dimensions of various
/// UI components and the terminal itself. Both dimensions use 0-based indexing,
/// where width represents the number of columns and height represents the number of rows.
///
/// # Examples
///
/// ```
/// let terminal_size = Size { width: 80, height: 24 };
/// let component_size = Size::default(); // { width: 0, height: 0 }
/// ```
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct Size {
    /// Height dimension (number of rows)
    pub height: usize,
    /// Width dimension (number of columns)
    pub width: usize,
}
