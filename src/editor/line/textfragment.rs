use super::GraphemeWidth;
/// Represents a fragment of text in a line.
#[derive(Clone)]
pub struct TextFragment {
    /// The grapheme of the fragment.
    pub grapheme: String,
    /// The rendered width of the fragment.
    pub rendered_width: GraphemeWidth,
    /// The replacement character for the fragment, if any.
    pub replacement: Option<char>,
    pub start_byte_idx: usize,
}
