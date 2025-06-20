//! # Annotated String Iterator Module
//!
//! This module provides an iterator implementation for AnnotatedString that
//! allows traversing the string while yielding each segment with its associated
//! annotation information.

use std::cmp::min;

use super::{AnnotatedString, AnnotatedStringPart};

/// Iterator for traversing annotated strings segment by segment
///
/// AnnotatedStringIterator provides a way to iterate over an AnnotatedString,
/// yielding AnnotatedStringPart objects that represent contiguous segments
/// of text with consistent annotation styling. This allows consumers to
/// process styled text piece by piece.
///
/// # Behavior
///
/// The iterator processes the string from left to right, yielding:
/// - Annotated segments where text has styling applied
/// - Non-annotated segments where text has no styling
/// - Segments are contiguous and non-overlapping
/// - The entire string content is covered exactly once
pub struct AnnotatedStringIterator<'a> {
    /// Reference to the annotated string being iterated
    pub annotated_string: &'a AnnotatedString,
    /// Current byte index position in the iteration
    pub current_idx: usize,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;

    /// Returns the next annotated string part
    ///
    /// This method processes the string to find the next contiguous segment
    /// that has consistent annotation styling. It handles overlapping annotations
    /// by giving precedence to the last (most recently added) annotation.
    ///
    /// # Returns
    ///
    /// - `Some(AnnotatedStringPart)` if there are more segments to process
    /// - `None` if the end of the string has been reached
    fn next(&mut self) -> Option<Self::Item> {
        // Check if we've reached the end of the string
        if self.current_idx >= self.annotated_string.string.len() {
            return None;
        }

        // Look for annotations that cover the current position
        if let Some(annotation) = self
            .annotated_string
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.start_byte_idx <= self.current_idx
                    && annotation.end_byte_idx > self.current_idx
            })
            .last()
        // Use the last annotation if multiple overlap (precedence)
        {
            // Found an annotation covering current position
            let end_idx = min(annotation.end_byte_idx, self.annotated_string.string.len());
            let start_idx = self.current_idx;
            self.current_idx = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotation_type: Some(annotation.annotation_type),
            });
        }

        // No annotation at current position, find the next unannotated segment
        let mut end_idx = self.annotated_string.string.len();
        for annotation in &self.annotated_string.annotations {
            if annotation.start_byte_idx > self.current_idx && annotation.start_byte_idx < end_idx {
                end_idx = annotation.start_byte_idx;
            }
        }

        let start_idx = self.current_idx;
        self.current_idx = end_idx;
        return Some(AnnotatedStringPart {
            string: &self.annotated_string.string[start_idx..end_idx],
            annotation_type: None,
        });
    }
}
