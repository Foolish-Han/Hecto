use std::{char, fmt, ops::Range};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Represents the width of a grapheme.
#[derive(Clone, Copy)]
enum GraphemeWidth {
    /// Half-width grapheme.
    Half,
    /// Full-width grapheme.
    Full,
}

impl GraphemeWidth {
    /// Adds the width of the grapheme to the given value.
    ///
    /// # Arguments
    ///
    /// * `other` - The value to add the width to.
    ///
    /// # Returns
    ///
    /// The result of adding the width to the given value.
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

/// Represents a fragment of text in a line.
struct TextFragment {
    /// The grapheme of the fragment.
    grapheme: String,
    /// The rendered width of the fragment.
    rendered_width: GraphemeWidth,
    /// The replacement character for the fragment, if any.
    replacement: Option<char>,
    start_byte_idx: usize,
}

/// Represents a line of text.
#[derive(Default)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

impl Line {
    /// Creates a new `Line` instance from a string.
    ///
    /// # Arguments
    ///
    /// * `line_str` - The string to create the line from.
    ///
    /// # Returns
    ///
    /// A new `Line` instance.
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments,
            string: String::from(line_str),
        }
    }

    /// Converts a string to a vector of text fragments.
    ///
    /// # Arguments
    ///
    /// * `line_str` - The string to convert.
    ///
    /// # Returns
    ///
    /// A vector of text fragments.
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
                    start_byte_idx: byte_idx,
                }
            })
            .collect()
    }

    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }

    /// Returns the replacement character for a given string, if any.
    ///
    /// # Arguments
    ///
    /// * `for_str` - The string to get the replacement character for.
    ///
    /// # Returns
    ///
    /// An option containing the replacement character, or `None` if no replacement is needed.
    fn get_replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }

    /// Returns the visible graphemes in the specified range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of graphemes to get.
    ///
    /// # Returns
    ///
    /// A string containing the visible graphemes in the specified range.
    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }
        let mut result = String::new();
        let mut current_pos = 0;
        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end {
                break;
            }
            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    // Clip on the right or left
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            current_pos = fragment_end;
        }
        result
    }

    /// Returns the number of graphemes in the line.
    ///
    /// # Returns
    ///
    /// The number of graphemes in the line.
    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    /// Returns the width of the line up to the specified grapheme index.
    ///
    /// # Arguments
    ///
    /// * `at` - The grapheme index to measure the width up to.
    ///
    /// # Returns
    ///
    /// The width of the line up to the specified grapheme index.
    pub fn width_until(&self, at: usize) -> usize {
        self.fragments
            .iter()
            .take(at)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn width(&self) -> usize {
        self.width_until(self.grapheme_count())
    }

    // Inserts  a character into the line, or appends it at the end if at > the len of the string
    /// Inserts a character at the specified grapheme index.
    ///
    /// # Arguments
    ///
    /// * `character` - The character to insert.
    /// * `at` - The grapheme index to insert the character at.
    pub fn insert_char(&mut self, character: char, at: usize) {
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start_byte_idx, character);
        } else {
            self.string.push(character);
        }
        self.rebuild_fragments();
    }

    pub fn append_char(&mut self, character: char) {
        self.insert_char(character, self.grapheme_count());
    }

    /// Deletes the grapheme at the specified index.
    ///
    /// # Arguments
    ///
    /// * `grapheme_index` - The index of the grapheme to delete.
    pub fn delete(&mut self, grapheme_index: usize) {
        if let Some(fragment) = self.fragments.get(grapheme_index) {
            let start = fragment.start_byte_idx;
            let end = start.saturating_add(fragment.grapheme.len());
            self.string.drain(start..end);
            self.rebuild_fragments();
        }
    }

    pub fn delete_last(&mut self) {
        self.delete(self.grapheme_count().saturating_sub(1));
    }
    /// Appends another line to this line.
    ///
    /// # Arguments
    ///
    /// * `other` - The line to append.
    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.rebuild_fragments();
    }

    /// Splits the line at the specified grapheme index.
    ///
    /// # Arguments
    ///
    /// * `at` - The grapheme index to split the line at.
    ///
    /// # Returns
    ///
    /// A new `Line` instance containing the remainder of the line after the split.
    pub fn split(&mut self, at: usize) -> Self {
        if let Some(fragment) = self.fragments.get(at) {
            let remainder = self.string.split_off(fragment.start_byte_idx);
            self.rebuild_fragments();
            Self::from(&remainder)
        } else {
            Self::default()
        }
    }

    fn byte_idx_to_grapheme_idx(&self, byte_idx: usize) -> usize {
        for (grapheme_idx, fragment) in self.fragments.iter().enumerate() {
            if fragment.start_byte_idx >= byte_idx {
                return grapheme_idx;
            }
        }
        #[cfg(debug_assertions)]
        {
            panic!("Invalid byte_idx passed to byte_idx_to_grapheme_idx: {byte_idx:?}");
        }
        #[cfg(not(debug_assertions))]
        {
            0
        }
    }

    pub fn search(&self, query: &str) -> Option<usize> {
        self.string
            .find(query)
            .map(|byte_idx| self.byte_idx_to_grapheme_idx(byte_idx))
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}
