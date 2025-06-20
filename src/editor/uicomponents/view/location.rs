//! Text location representation for cursor positioning.
//!
//! This module provides the `Location` struct, which represents a specific
//! position within a text document using line and grapheme indices. This
//! location system is Unicode-aware and properly handles grapheme clusters.
//!
//! # Examples
//!
//! ```rust
//! use hecto::editor::uicomponents::view::location::Location;
//!
//! // Create a location at the beginning of the document
//! let start = Location { line_idx: 0, grapheme_idx: 0 };
//!
//! // Create a location at line 5, character 10
//! let cursor = Location { line_idx: 5, grapheme_idx: 10 };
//! ```

/// Represents a specific location within a text document.
///
/// A `Location` identifies a precise position in a document using zero-based
/// line and grapheme indices. The grapheme index represents the position within
/// a line counting Unicode grapheme clusters rather than bytes or code points,
/// ensuring proper handling of complex Unicode text.
///
/// # Fields
///
/// - `line_idx`: Zero-based index of the line within the document
/// - `grapheme_idx`: Zero-based index of the grapheme cluster within the line
///
/// # Unicode Handling
///
/// The `grapheme_idx` counts grapheme clusters, which means:
/// - Multi-byte UTF-8 characters count as one position
/// - Combining characters are grouped with their base character
/// - Emoji sequences are treated as single units
///
/// # Examples
///
/// ```rust
/// use hecto::editor::uicomponents::view::location::Location;
///
/// // Beginning of document
/// let start = Location::default();
/// assert_eq!(start.line_idx, 0);
/// assert_eq!(start.grapheme_idx, 0);
///
/// // Specific position
/// let position = Location { line_idx: 2, grapheme_idx: 15 };
/// println!("Line {}, Column {}", position.line_idx + 1, position.grapheme_idx + 1);
/// ```
#[derive(Copy, Clone, Default)]
pub struct Location {
    /// Zero-based index of the grapheme cluster within the line
    pub grapheme_idx: usize,
    /// Zero-based index of the line within the document
    pub line_idx: usize,
}
