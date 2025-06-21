//! # Text Fragment Module
//!
//! This module defines the TextFragment structure, which represents a single
//! Unicode grapheme within a line of text along with its display properties
//! and metadata.

use crate::prelude::*;

use super::GraphemeWidth;

/// Represents a single grapheme cluster within a line of text
///
/// A TextFragment contains all the information needed to properly display
/// and manipulate a Unicode grapheme, including its original form, display
/// width, any replacement character for display purposes, and its position
/// within the original string.
///
/// This structure is essential for proper Unicode handling, as it allows
/// the editor to work with grapheme clusters (which may consist of multiple
/// Unicode code points) as single units while maintaining proper display
/// formatting and positioning.
#[derive(Clone)]
pub struct TextFragment {
    /// The original grapheme cluster as a string
    pub grapheme: String,
    /// The display width of this grapheme in terminal columns
    pub rendered_width: GraphemeWidth,
    /// Optional replacement character for display (e.g., for control characters)
    pub replacement: Option<char>,
    /// Byte index where this grapheme starts in the original string
    pub start: ByteIdx,
}
