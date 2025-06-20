//! # Grapheme Width Module
//!
//! This module defines the GraphemeWidth enum used to represent the display width
//! of Unicode graphemes in terminal environments. Some characters take up more
//! terminal space than others, particularly wide characters like certain emojis
//! and CJK (Chinese, Japanese, Korean) characters.

/// Represents the display width of a grapheme in terminal columns
///
/// Different Unicode graphemes take up different amounts of space when displayed
/// in a terminal. This enum categorizes them into two main types:
/// - Half-width characters (normal ASCII characters, most symbols)
/// - Full-width characters (wide Unicode characters, some emojis, CJK characters)
#[derive(Clone, Copy, Debug)]
pub enum GraphemeWidth {
    /// Half-width characters that occupy 1 terminal column
    Half,
    /// Full-width characters that occupy 2 terminal columns
    Full,
}

impl From<GraphemeWidth> for usize {
    /// Converts a GraphemeWidth to the number of terminal columns it occupies
    ///
    /// # Arguments
    ///
    /// * `value` - The GraphemeWidth to convert
    ///
    /// # Returns
    ///
    /// Returns 1 for Half-width characters and 2 for Full-width characters
    ///
    /// # Examples
    ///
    /// ```
    /// let half_width = GraphemeWidth::Half;
    /// let full_width = GraphemeWidth::Full;
    ///
    /// assert_eq!(usize::from(half_width), 1);
    /// assert_eq!(usize::from(full_width), 2);
    /// ```
    fn from(value: GraphemeWidth) -> Self {
        match value {
            GraphemeWidth::Full => 2,
            GraphemeWidth::Half => 1,
        }
    }
}
