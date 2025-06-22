//! # Line Module
//!
//! This module provides the Line structure and associated functionality for representing
//! and manipulating individual lines of text in the editor. It handles Unicode text
//! properly by working with grapheme clusters rather than individual bytes or characters.
//!
//! ## Key Features
//!
//! - **Unicode Support**: Proper handling of Unicode grapheme clusters, including
//!   combining characters, emojis, and multi-byte characters
//! - **Display Width Calculation**: Accurate calculation of how much terminal space
//!   each character will occupy, including wide characters
//! - **Character Replacement**: Visual replacement of control characters and
//!   whitespace for better editing experience
//! - **Search Functionality**: Built-in forward and backward text search within lines
//! - **Efficient Editing**: Support for insertion, deletion, and splitting operations
//! - **Annotation Support**: Integration with the annotation system for highlighting
//!   search results and other visual effects
//!
//! ## Design
//!
//! The Line struct maintains both the original string content and a parsed
//! representation as TextFragment objects. This dual representation allows for
//! efficient string operations while maintaining accurate position tracking
//! and display width calculations.

mod graphemewidth;
mod textfragment;

use crate::{
    editor::annotation::{self, Annotation},
    prelude::*,
};

use std::{
    cmp::min,
    fmt::{self, Display},
    ops::{Deref, Range},
};

use graphemewidth::GraphemeWidth;
use textfragment::TextFragment;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::{AnnotatedString, AnnotationType};

/// Represents a single line of text with Unicode support
///
/// Line provides a rich representation of a text line that properly handles
/// Unicode grapheme clusters, display width calculations, and visual formatting.
/// It maintains both the original string and a parsed fragment representation
/// for efficient operations.
///
/// # Examples
///
/// ```
/// let line = Line::from("Hello, 世界! 👋");
/// println!("Width: {}", line.width());
/// println!("Graphemes: {}", line.grapheme_count());
/// ```
#[derive(Default, Clone)]
pub struct Line {
    /// Parsed grapheme fragments with display information
    fragments: Vec<TextFragment>,
    /// Original string content
    string: String,
}
impl Line {
    /// Creates a new Line from a string
    ///
    /// This method parses the input string into grapheme clusters and creates
    /// the corresponding TextFragment objects. It validates that the input
    /// contains at most one line (no newline characters except potentially
    /// at the end).
    ///
    /// # Arguments
    ///
    /// * `line_str` - The string content for the line
    ///
    /// # Returns
    ///
    /// A new Line instance containing the parsed content
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the input string contains multiple lines
    ///
    /// # Examples
    ///
    /// ```
    /// let line = Line::from("Hello, world!");
    /// let empty_line = Line::from("");
    /// let unicode_line = Line::from("Hello, 世界! 👋");
    /// ```
    pub fn from(line_str: &str) -> Self {
        debug_assert!(line_str.is_empty() || line_str.lines().count() == 1);
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments,
            string: String::from(line_str),
        }
    }

    /// Converts a string into TextFragment objects
    ///
    /// This method processes each grapheme cluster in the string and creates
    /// corresponding TextFragment objects with proper display width calculation
    /// and replacement character handling.
    ///
    /// # Arguments
    ///
    /// * `line_str` - The string to process
    ///
    /// # Returns
    ///
    /// A vector of TextFragment objects representing the string
    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .grapheme_indices(true)
            .map(|(byte_idx, grapheme)| {
                let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );
                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    start: byte_idx,
                }
            })
            .collect()
    }
    /// Rebuilds the fragment list from the current string content
    ///
    /// This method is called after string modifications to ensure the fragment
    /// list stays in sync with the string content.
    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }

    /// Determines if a character should be replaced for display purposes
    ///
    /// This method handles visual representation of special characters:
    /// - Regular spaces are not replaced
    /// - Tabs are replaced with space characters
    /// - Other whitespace characters are replaced with a visible symbol (␣)
    /// - Zero-width characters are replaced with appropriate symbols (▯ for control chars, · for others)
    /// - Regular printable characters are not replaced
    ///
    /// # Arguments
    ///
    /// * `for_str` - The string (grapheme) to check
    ///
    /// # Returns
    ///
    /// `Some(char)` if the string should be replaced with a visible character,
    /// `None` if it should be displayed as-is
    fn get_replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,       // Regular space - no replacement
            "\t" => Some(' '), // Tab - replace with space
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'), // Other whitespace
            _ if width == 0 => {
                // Zero-width characters
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯'); // Control characters
                    }
                }
                Some('·') // Other zero-width characters
            },
            _ => None, // Regular printable characters
        }
    }
    /// Gets the visible portion of the line within a column range
    ///
    /// This method extracts the visible characters within the specified column
    /// range, applying any necessary character replacements for display purposes.
    /// This is a convenience method that calls `get_annotated_visible_substr`
    /// without annotations.
    ///
    /// # Arguments
    ///
    /// * `range` - The column range to extract (start..end)
    ///
    /// # Returns
    ///
    /// A String containing the visible text within the specified range
    pub fn get_visible_graphemes(&self, range: Range<ColIdx>) -> String {
        self.get_annotated_visible_substr(range, None).to_string()
    }

    /// Gets an annotated substring with search highlighting and visual formatting
    ///
    /// This method extracts a portion of the line within the specified column range
    /// and applies appropriate visual formatting including:
    /// - Character replacement for special characters
    /// - Search result highlighting
    /// - Truncation indicators (⋯) when text extends beyond the visible area
    ///
    /// # Arguments
    ///
    /// * `range` - The column range to extract (start..end)
    /// * `query` - Optional search query to highlight within the text
    /// * `selected_match` - Optional index of the currently selected search match
    ///
    /// # Returns
    ///
    /// An AnnotatedString with appropriate styling and annotations applied
    pub fn get_annotated_visible_substr(
        &self,
        range: Range<ColIdx>,
        annotations: Option<&Vec<Annotation>>,
    ) -> AnnotatedString {
        if range.start >= range.end {
            return AnnotatedString::default();
        }

        let mut result = AnnotatedString::from(&self.string);

        if let Some(annotations) = annotations {
            for annotation in annotations {
                result.add_annotation(annotation.annotation_type, annotation.start, annotation.end);
            }
        }

        // Process fragments to handle viewport clipping and character replacement
        let mut fragment_start = self.width();
        for fragment in self.fragments.iter().rev() {
            let fragment_end = fragment_start;
            fragment_start = fragment_end.saturating_sub(fragment.rendered_width.into());

            // Skip fragments that are completely to the right of the viewport
            if fragment_start > range.end {
                continue;
            }

            // Handle fragments that extend beyond the right edge of the viewport
            if fragment_start < range.end && fragment_end > range.end {
                result.replace(fragment.start, self.string.len(), "⋯");
                continue;
            } else if fragment_start == range.end {
                // Remove fragments at the exact right boundary
                result.truncate_right_from(fragment.start);
                continue;
            }

            // Handle fragments that are completely to the left of the viewport
            if fragment_end <= range.start {
                result.truncate_left_until(fragment.start.saturating_add(fragment.grapheme.len()));
                break;
            } else if fragment_start < range.start && fragment_end > range.start {
                // Handle fragments that extend beyond the left edge of the viewport
                result.replace(
                    0,
                    fragment.start.saturating_add(fragment.grapheme.len()),
                    "⋯",
                );
                break;
            }

            // Apply character replacement for fragments within the visible range
            if fragment_start >= range.start && fragment_end <= range.end {
                if let Some(replacement) = fragment.replacement {
                    let start = fragment.start;
                    let end = start.saturating_add(fragment.grapheme.len());
                    result.replace(start, end, &replacement.to_string());
                }
            }
        }

        result
    }
    /// Returns the number of grapheme clusters in the line
    ///
    /// # Returns
    ///
    /// The total number of grapheme clusters (visual characters) in the line
    pub fn grapheme_count(&self) -> GraphemeIdx {
        self.fragments.len()
    }

    /// Calculates the display width up to a specific grapheme index
    ///
    /// This method sums the display widths of all graphemes from the beginning
    /// of the line up to (but not including) the specified grapheme index.
    ///
    /// # Arguments
    ///
    /// * `grapheme_idx` - The grapheme index to calculate width up to
    ///
    /// # Returns
    ///
    /// The total display width in terminal columns
    pub fn width_until(&self, grapheme_idx: GraphemeIdx) -> ColIdx {
        self.fragments
            .iter()
            .take(grapheme_idx)
            .map(|fragment| usize::from(fragment.rendered_width))
            .sum()
    }

    /// Returns the total display width of the line
    ///
    /// # Returns
    ///
    /// The total width of the line in terminal columns
    pub fn width(&self) -> ColIdx {
        self.width_until(self.grapheme_count())
    }

    /// Inserts a character at the specified grapheme position
    ///
    /// This method inserts a character at the given grapheme index, automatically
    /// handling the byte-level insertion and rebuilding the fragment list.
    ///
    /// # Arguments
    ///
    /// * `character` - The character to insert
    /// * `at` - The grapheme index where to insert the character
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the insertion position is invalid
    pub fn insert_char(&mut self, character: char, at: GraphemeIdx) {
        debug_assert!(at.saturating_sub(1) <= self.grapheme_count());
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start, character);
        } else {
            self.string.push(character);
        }
        self.rebuild_fragments();
    }

    /// Appends a character to the end of the line
    ///
    /// This is a convenience method that inserts a character at the end of the line.
    ///
    /// # Arguments
    ///
    /// * `character` - The character to append
    pub fn append_char(&mut self, character: char) {
        self.insert_char(character, self.grapheme_count());
    }

    /// Deletes the grapheme at the specified position
    ///
    /// This method removes the grapheme cluster at the given index, handling
    /// the byte-level deletion and rebuilding the fragment list.
    ///
    /// # Arguments
    ///
    /// * `at` - The grapheme index of the character to delete
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the deletion position is invalid
    pub fn delete(&mut self, at: GraphemeIdx) {
        debug_assert!(at <= self.grapheme_count());
        if let Some(fragment) = self.fragments.get(at) {
            let start = fragment.start;
            let end = start.saturating_add(fragment.grapheme.len());
            self.string.drain(start..end);
            self.rebuild_fragments();
        }
    }

    /// Deletes the last grapheme in the line
    ///
    /// This is a convenience method for deleting the character at the end of the line.
    pub fn delete_last(&mut self) {
        self.delete(self.grapheme_count().saturating_sub(1));
    }
    /// Appends the content of another line to this line
    ///
    /// This method concatenates the string content of another line to the end
    /// of this line and rebuilds the fragment list.
    ///
    /// # Arguments
    ///
    /// * `other` - The line whose content should be appended
    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.rebuild_fragments();
    }

    /// Splits the line at the specified grapheme position
    ///
    /// This method splits the line into two parts at the given grapheme index.
    /// The current line retains the content before the split point, and a new
    /// line containing the content after the split point is returned.
    ///
    /// # Arguments
    ///
    /// * `at` - The grapheme index where to split the line
    ///
    /// # Returns
    ///
    /// A new Line containing the content after the split point
    pub fn split(&mut self, at: GraphemeIdx) -> Self {
        if let Some(fragment) = self.fragments.get(at) {
            let remainder = self.string.split_off(fragment.start);
            self.rebuild_fragments();
            Self::from(&remainder)
        } else {
            Self::default()
        }
    }

    /// Converts a byte index to a grapheme index
    ///
    /// This method finds the grapheme that contains or starts at the given byte index.
    ///
    /// # Arguments
    ///
    /// * `byte_idx` - The byte index to convert
    ///
    /// # Returns
    ///
    /// `Some(GraphemeIdx)` if a valid grapheme is found, `None` if the byte index
    /// is beyond the string length
    fn byte_idx_to_grapheme_idx(&self, byte_idx: ByteIdx) -> Option<GraphemeIdx> {
        if byte_idx > self.string.len() {
            return None;
        }
        self.fragments
            .iter()
            .position(|fragment| fragment.start >= byte_idx)
    }

    /// Converts a grapheme index to the corresponding byte index
    ///
    /// This method returns the byte position where the specified grapheme starts.
    ///
    /// # Arguments
    ///
    /// * `grapheme_idx` - The grapheme index to convert
    ///
    /// # Returns
    ///
    /// The byte index where the grapheme starts
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the grapheme index is invalid
    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx {
        debug_assert!(grapheme_idx <= self.grapheme_count());
        if grapheme_idx == 0 || self.grapheme_count() == 0 {
            return 0;
        }
        self.fragments.get(grapheme_idx).map_or_else(
            || {
                #[cfg(debug_assertions)]
                {
                    panic!("Fragment not found for grapheme index: {grapheme_idx:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    0
                }
            },
            |fragment| fragment.start,
        )
    }

    /// Searches for a query string in the forward direction
    ///
    /// This method searches for the specified query string starting from the given
    /// grapheme index and moving toward the end of the line.
    ///
    /// # Arguments
    ///
    /// * `query` - The string to search for
    /// * `from_grapheme_idx` - The grapheme index to start searching from
    ///
    /// # Returns
    ///
    /// `Some(GraphemeIdx)` if a match is found, `None` if no match is found
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the starting grapheme index is invalid
    pub fn search_forward(
        &self,
        query: &str,
        from_grapheme_idx: GraphemeIdx,
    ) -> Option<GraphemeIdx> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == self.grapheme_count() {
            return None;
        }
        let start = self.grapheme_idx_to_byte_idx(from_grapheme_idx);
        self.find_all(query, start..self.string.len())
            .first()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    /// Searches for a query string in the backward direction
    ///
    /// This method searches for the specified query string starting from the given
    /// grapheme index and moving toward the beginning of the line.
    ///
    /// # Arguments
    ///
    /// * `query` - The string to search for
    /// * `from_grapheme_idx` - The grapheme index to start searching from
    ///
    /// # Returns
    ///
    /// `Some(GraphemeIdx)` if a match is found, `None` if no match is found
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the starting grapheme index is invalid
    pub fn search_backward(
        &self,
        query: &str,
        from_grapheme_idx: GraphemeIdx,
    ) -> Option<GraphemeIdx> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == 0 {
            return None;
        }
        let end_byte_idx = if from_grapheme_idx == self.grapheme_count() {
            self.string.len()
        } else {
            self.grapheme_idx_to_byte_idx(from_grapheme_idx)
        };
        self.find_all(query, 0..end_byte_idx)
            .last()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    /// Finds all occurrences of a query string within a byte range
    ///
    /// This method searches for all occurrences of the query string within the
    /// specified byte range and returns both the byte positions and corresponding
    /// grapheme indices.
    ///
    /// # Arguments
    ///
    /// * `query` - The string to search for
    /// * `range` - The byte range to search within
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (byte_index, grapheme_index) for each match
    pub fn find_all(&self, query: &str, range: Range<ByteIdx>) -> Vec<(ByteIdx, GraphemeIdx)> {
        let end = min(range.end, self.string.len());
        let start = range.start;
        debug_assert!(start <= end);
        self.string.get(start..end).map_or_else(
            || Vec::new(),
            |substr| {
                let potential_matches: Vec<ByteIdx> = substr
                    .match_indices(query)
                    .map(|(relative_start_idx, _)| relative_start_idx.saturating_add(start))
                    .collect();
                self.match_grapheme_clusters(&potential_matches, query)
            },
        )
    }

    fn match_grapheme_clusters(
        &self,
        matches: &[ByteIdx],
        query: &str,
    ) -> Vec<(ByteIdx, GraphemeIdx)> {
        let grapheme_count = query.graphemes(true).count();
        matches
            .iter()
            .filter_map(|&start| {
                self.byte_idx_to_grapheme_idx(start)
                    .and_then(|grapheme_idx| {
                        self.fragments
                            .get(grapheme_idx..grapheme_idx.saturating_add(grapheme_count))
                            .and_then(|fragments| {
                                let substring = fragments
                                    .iter()
                                    .map(|fragment| fragment.grapheme.as_str())
                                    .collect::<String>();
                                (substring == query).then_some((start, grapheme_idx))
                            })
                    })
            })
            .collect()
    }
}

/// Display implementation for Line
///
/// This allows Line objects to be formatted as strings, displaying their
/// raw string content without any special formatting or replacement characters.
impl Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

/// Deref implementation for Line
///
/// This allows Line objects to be used as string slices, providing convenient
/// access to string methods and operations on the underlying string content.
impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
