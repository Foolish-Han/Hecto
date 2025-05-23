/// Represents a location in the text buffer.
#[derive(Copy, Clone, Default)]
pub struct Location {
    /// The index of the grapheme in the line.
    pub grapheme_idx: usize,
    /// The index of the line in the buffer.
    pub line_idx: usize,
}
