//! # Annotated String Part Module
//!
//! This module defines the AnnotatedStringPart structure which represents
//! a portion of an annotated string with its associated annotation type.

use super::AnnotationType;

/// Represents a portion of an annotated string with optional styling
///
/// An AnnotatedStringPart is a view into a section of an annotated string,
/// containing a string slice and an optional annotation type. This structure
/// is used when iterating over annotated strings to process each styled
/// segment individually.
///
/// # Lifetime
///
/// The lifetime parameter `'a` ensures that the string slice reference
/// remains valid for the duration of the part's existence.
#[derive(Debug)]
pub struct AnnotatedStringPart<'a> {
    /// String slice containing the text content
    pub string: &'a str,
    /// Optional annotation type for styling this text segment
    pub annotation_type: Option<AnnotationType>,
}
