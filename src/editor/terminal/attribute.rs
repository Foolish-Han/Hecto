//! # Terminal Attribute Module
//!
//! This module defines text attributes for terminal display, including foreground
//! and background colors. It provides mapping from annotation types to visual
//! styling attributes.

use crossterm::style::Color;

use super::super::annotatedstring::AnnotationType;

/// Represents terminal text display attributes
///
/// Attribute defines how text should be displayed in the terminal, including
/// foreground and background colors. This is used to provide visual feedback
/// for search results and other highlighted text.
pub struct Attribute {
    /// Optional foreground (text) color
    pub foreground: Option<Color>,
    /// Optional background color
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    /// Converts an annotation type to display attributes
    ///
    /// This method maps different annotation types to their corresponding
    /// visual styling:
    /// - `Match`: Regular search matches with white text on gray background
    /// - `SelectedMatch`: Currently selected search match with white text on yellow background
    ///
    /// # Arguments
    ///
    /// * `value` - The annotation type to convert
    ///
    /// # Returns
    ///
    /// An Attribute struct with the appropriate colors set
    fn from(value: AnnotationType) -> Self {
        match value {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 211,
                    g: 211,
                    b: 211,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 153,
                }),
            },
        }
    }
}
