//! # Annotation Module
//!
//! This module defines the Annotation structure which represents a single
//! text annotation with its type and byte range within a string.

use crate::prelude::*;

use super::AnnotationType;

/// Represents a single annotation applied to a range of text
///
/// An Annotation defines a visual styling that should be applied to a specific
/// byte range within a string. It contains the type of annotation (which determines
/// the visual appearance) and the start and end positions in bytes.
///
/// # Examples
///
/// ```
/// let annotation = Annotation {
///     annotation_type: AnnotationType::Match,
///     start_byte_idx: 5,
///     end_byte_idx: 10,
/// };
/// // This would highlight bytes 5-9 as a search match
/// ```
#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
    /// The type of annotation, determining visual appearance
    pub annotation_type: AnnotationType,
    /// Starting byte index (inclusive) of the annotation
    pub start: ByteIdx,
    /// Ending byte index (exclusive) of the annotation
    pub end: ByteIdx,
}
