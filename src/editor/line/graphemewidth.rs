#[derive(Clone, Copy, Debug)]
pub enum GraphemeWidth {
    Half,
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
