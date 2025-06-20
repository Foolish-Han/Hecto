//! # Annotation Type Module
//!
//! This module defines the types of annotations that can be applied to text
//! for visual highlighting and formatting purposes.

/// Represents different types of text annotations for visual highlighting
///
/// AnnotationType defines the various ways text can be annotated for display,
/// primarily used for search result highlighting. Each type corresponds to
/// a different visual style that will be applied in the terminal.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AnnotationType {
    /// Regular search match highlighting
    ///
    /// Used to highlight text that matches a search query with standard
    /// highlighting (typically white text on gray background).
    Match,

    /// Currently selected search match highlighting
    ///
    /// Used to highlight the currently active search match with more prominent
    /// highlighting (typically white text on yellow background) to distinguish
    /// it from other matches.
    SelectedMatch,
}
