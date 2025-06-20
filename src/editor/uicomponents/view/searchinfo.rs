//! Search state information for the text editor.
//!
//! This module provides the `SearchInfo` struct, which maintains the state
//! of active search operations. It stores the original cursor position and
//! scroll offset to enable returning to the pre-search state if needed.
//!
//! # Examples
//!
//! ```rust
//! use hecto::editor::uicomponents::view::{SearchInfo, Location};
//! use hecto::editor::{Position, Line};
//!
//! let search_info = SearchInfo {
//!     prev_location: Location { line_idx: 5, grapheme_idx: 10 },
//!     prev_scroll_offset: Position { row: 0, col: 0 },
//!     query: Some(Line::from("search_term")),
//! };
//! ```

use super::Location;
use crate::editor::{Line, Position};

/// Maintains state information for active search operations.
///
/// `SearchInfo` stores the necessary information to manage search operations,
/// including the ability to return to the original position if the search
/// is cancelled. This allows users to dismiss a search and return to their
/// original location in the document.
///
/// # Fields
///
/// - `prev_location`: The cursor location before the search began
/// - `prev_scroll_offset`: The scroll position before the search began
/// - `query`: The current search query, if any
///
/// # Usage
///
/// The search info is typically created when entering search mode and used
/// to restore the previous state when exiting or dismissing the search.
///
/// # Examples
///
/// ```rust
/// use hecto::editor::uicomponents::view::{SearchInfo, Location};
/// use hecto::editor::{Position, Line};
///
/// // Create search info when starting a search
/// let search_info = SearchInfo {
///     prev_location: Location { line_idx: 10, grapheme_idx: 5 },
///     prev_scroll_offset: Position { row: 5, col: 0 },
///     query: None, // Will be set when user enters search term
/// };
///
/// // Later, set the query
/// let mut search_info = search_info;
/// search_info.query = Some(Line::from("hello"));
/// ```
pub struct SearchInfo {
    /// The cursor location before the search operation began
    pub prev_location: Location,
    /// The scroll offset before the search operation began
    pub prev_scroll_offset: Position,
    /// The current search query, if one has been entered
    pub query: Option<Line>,
}
