use crate::editor::{Line, Position};

use super::Location;

/// Represents information about a search operation.
pub struct SearchInfo {
    /// The previous location before the search.
    pub prev_location: Location,
    /// The previous scroll offset before the search.
    pub prev_scroll_offset: Position,
    /// The query string for the search.
    pub query: Option<Line>,
}
