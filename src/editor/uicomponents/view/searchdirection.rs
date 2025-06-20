//! Search direction enumeration for text search operations.
//!
//! This module provides the `SearchDirection` enum, which specifies the
//! direction for text search operations within the editor. It supports
//! both forward and backward search directions.
//!
//! # Examples
//!
//! ```rust
//! use hecto::editor::uicomponents::view::searchdirection::SearchDirection;
//!
//! // Default direction is forward
//! let direction = SearchDirection::default();
//! assert_eq!(direction, SearchDirection::Forward);
//!
//! // Explicit backward direction
//! let backward = SearchDirection::Backward;
//! ```

/// Specifies the direction for text search operations.
///
/// `SearchDirection` is used to control whether search operations proceed
/// forward (towards the end of the document) or backward (towards the
/// beginning of the document). Forward search is the default behavior.
///
/// # Variants
///
/// - `Forward`: Search from the current position towards the end of the document
/// - `Backward`: Search from the current position towards the beginning of the document
///
/// # Examples
///
/// ```rust
/// use hecto::editor::uicomponents::view::searchdirection::SearchDirection;
///
/// // Default search direction
/// let default_dir = SearchDirection::default();
/// assert_eq!(default_dir, SearchDirection::Forward);
///
/// // Compare directions
/// if direction == SearchDirection::Forward {
///     println!("Searching forward");
/// } else {
///     println!("Searching backward");
/// }
/// ```
#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub enum SearchDirection {
    /// Search forward towards the end of the document (default)
    #[default]
    Forward,
    /// Search backward towards the beginning of the document
    Backward,
}
