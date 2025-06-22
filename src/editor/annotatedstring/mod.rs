//! # Annotated String Module
//!
//! This module provides functionality for creating and manipulating strings with
//! visual annotations. It allows text to be marked with different styling information
//! such as search match highlighting, which can then be rendered with appropriate
//! visual formatting in the terminal.
//!
//! ## Key Components
//!
//! - **AnnotatedString**: The main structure containing text and annotations
//! - **Annotation**: Individual styling annotations with type and byte range
//! - **AnnotationType**: Enum defining different types of visual styling
//! - **AnnotatedStringIterator**: Iterator for processing annotated text segments
//! - **AnnotatedStringPart**: Individual segments yielded by the iterator
//!
//! ## Usage
//!
//! Annotated strings are primarily used for search result highlighting, where
//! search matches are visually distinguished from regular text. The system
//! supports overlapping annotations with proper precedence handling.

use crate::prelude::*;

use std::{
    cmp::{max, min},
    fmt::{self, Display},
};

mod annotatedstringpart;
mod annotationstringiterator;

use super::{Annotation, AnnotationType};
use annotatedstringpart::AnnotatedStringPart;
use annotationstringiterator::AnnotatedStringIterator;
/// A string with associated visual annotations for styling and highlighting
///
/// AnnotatedString combines text content with styling annotations that define
/// how different portions of the text should be visually rendered. This is
/// primarily used for search result highlighting, where matches are visually
/// distinguished from regular text.
///
/// ## Features
///
/// - Supports multiple overlapping annotations
/// - Efficient text replacement with automatic annotation adjustment
/// - Iterator support for processing styled segments
/// - Automatic cleanup of invalid annotations
///
/// ## Examples
///
/// ```
/// let mut annotated = AnnotatedString::from("Hello world");
/// annotated.add_annotation(AnnotationType::Match, 6, 11);
/// // "world" is now annotated as a search match
/// ```
#[derive(Default, Debug)]
pub struct AnnotatedString {
    /// The text content
    string: String,
    /// List of annotations applied to the text
    annotations: Vec<Annotation>,
}
impl AnnotatedString {
    /// Creates a new AnnotatedString from a string slice
    ///
    /// This method creates an AnnotatedString with the provided text content
    /// and no initial annotations.
    ///
    /// # Arguments
    ///
    /// * `string` - The text content for the annotated string
    ///
    /// # Returns
    ///
    /// A new AnnotatedString instance with the provided content
    ///
    /// # Examples
    ///
    /// ```
    /// let annotated = AnnotatedString::from("Hello, world!");
    /// ```
    pub fn from(string: &str) -> Self {
        Self {
            string: String::from(string),
            annotations: Vec::new(),
        }
    }

    /// Adds an annotation to a specific byte range
    ///
    /// This method applies visual styling to a portion of the text defined
    /// by byte indices. The annotation will affect how the text is rendered
    /// when displayed.
    ///
    /// # Arguments
    ///
    /// * `annotation_type` - The type of annotation to apply
    /// * `start_byte_idx` - Starting byte index (inclusive)
    /// * `end_byte_idx` - Ending byte index (exclusive)
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the start index is greater than the end index
    ///
    /// # Examples
    ///
    /// ```
    /// let mut annotated = AnnotatedString::from("Hello world");
    /// annotated.add_annotation(AnnotationType::Match, 0, 5);
    /// // "Hello" is now highlighted as a match
    /// ```
    pub fn add_annotation(
        &mut self,
        annotation_type: AnnotationType,
        start: ByteIdx,
        end: ByteIdx,
    ) {
        debug_assert!(start <= end);
        self.annotations.push(Annotation {
            annotation_type,
            start,
            end,
        });
    }

    pub fn truncate_left_until(&mut self, until: ByteIdx) {
        self.replace(0, until, "");
    }

    pub fn truncate_right_from(&mut self, from: ByteIdx) {
        self.replace(from, self.string.len(), "");
    }

    /// Replaces a portion of the string and adjusts annotations accordingly
    ///
    /// This method replaces text within the specified byte range with new content
    /// and automatically adjusts all annotations to maintain their correct positions
    /// relative to the modified text. This is essential for maintaining annotation
    /// integrity when the underlying text is modified.
    ///
    /// # Behavior
    ///
    /// - Replaces the text in the specified range with the new string
    /// - Adjusts all annotation positions based on the length change
    /// - Removes annotations that become invalid after the replacement
    /// - Handles both text expansion and contraction properly
    ///
    /// # Arguments
    ///
    /// * `start_byte_idx` - Starting byte index of the range to replace
    /// * `end_byte_idx` - Ending byte index of the range to replace
    /// * `new_string` - The replacement text
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the start index is greater than the end index
    ///
    /// # Examples
    ///
    /// ```
    /// let mut annotated = AnnotatedString::from("Hello world");
    /// annotated.add_annotation(AnnotationType::Match, 6, 11);
    /// annotated.replace(6, 11, "everyone");
    /// // "world" is replaced with "everyone", annotation is adjusted
    /// ```
    pub fn replace(&mut self, start: ByteIdx, end: ByteIdx, new_string: &str) {
        let end = min(end, self.string.len());
        debug_assert!(start <= end);
        if start > end {
            return;
        }

        // Perform the string replacement
        self.string.replace_range(start..end, new_string);

        // Calculate the length change to adjust annotations
        let replaced_range_len = end.saturating_sub(start);
        let shortened = new_string.len() < replaced_range_len;
        let len_difference = new_string.len().abs_diff(replaced_range_len);

        // If no length change, no annotation adjustment needed
        if len_difference == 0 {
            return;
        }

        // Adjust all annotations based on the text change
        self.annotations.iter_mut().for_each(|annotation| {
            // Adjust start position
            annotation.start = if annotation.start >= end {
                // Annotation starts after the replacement - shift by length difference
                if shortened {
                    annotation.start.saturating_sub(len_difference)
                } else {
                    annotation.start.saturating_add(len_difference)
                }
            } else if annotation.start >= start {
                // Annotation starts within the replacement range - clamp to boundaries
                if shortened {
                    max(start, annotation.start.saturating_sub(len_difference))
                } else {
                    min(end, annotation.start.saturating_add(len_difference))
                }
            } else {
                // Annotation starts before the replacement - no change needed
                annotation.start
            };

            // Adjust end position using similar logic
            annotation.end = if annotation.end >= end {
                // Annotation ends after the replacement - shift by length difference
                if shortened {
                    annotation.end.saturating_sub(len_difference)
                } else {
                    annotation.end.saturating_add(len_difference)
                }
            } else if annotation.end >= start {
                // Annotation ends within the replacement range - clamp to boundaries
                if shortened {
                    max(start, annotation.end.saturating_sub(len_difference))
                } else {
                    min(end, annotation.end.saturating_add(len_difference))
                }
            } else {
                // Annotation ends before the replacement - no change needed
                annotation.end
            }
        });

        // Remove annotations that have become invalid (zero-length or out-of-bounds)
        self.annotations.retain(|annotation| {
            annotation.start < annotation.end && annotation.start < self.string.len()
        });
    }
}

/// Display implementation for AnnotatedString
///
/// This allows AnnotatedString to be formatted as a regular string,
/// displaying only the text content without any annotation information.
/// Annotations are purely metadata and don't affect the string representation.
impl Display for AnnotatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

/// IntoIterator implementation for AnnotatedString references
///
/// This enables AnnotatedString references to be used with for loops and
/// other iterator-based operations. The iterator yields AnnotatedStringPart
/// objects representing contiguous segments with consistent styling.
///
/// # Examples
///
/// ```
/// let annotated = AnnotatedString::from("Hello world");
/// for part in &annotated {
///     println!("Text: '{}', Styled: {}", part.string, part.annotation_type.is_some());
/// }
/// ```
impl<'a> IntoIterator for &'a AnnotatedString {
    type IntoIter = AnnotatedStringIterator<'a>;
    type Item = AnnotatedStringPart<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AnnotatedStringIterator {
            annotated_string: self,
            current_idx: 0,
        }
    }
}
