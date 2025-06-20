/// Represents the width of a grapheme.
#[derive(Clone, Copy, Debug)]
pub enum GraphemeWidth {
    /// Half-width grapheme.
    Half,
    /// Full-width grapheme.
    Full,
}

impl From<GraphemeWidth> for usize {
    fn from(value: GraphemeWidth) -> Self {
        match value {
            GraphemeWidth::Full => 2,
            GraphemeWidth::Half => 1,
        }
    }
}
