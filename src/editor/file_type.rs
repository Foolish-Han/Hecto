use std::{fmt::Display, path::PathBuf};
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    #[default]
    PlainText,
    Rust,
}

impl From<&PathBuf> for FileType {
    fn from(path_buf: &PathBuf) -> Self {
        path_buf
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "rs" => FileType::Rust,
                _ => FileType::PlainText,
            })
            .unwrap_or(FileType::PlainText)
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Rust => "Rust",
            _ => "Text",
        };
        write!(f, "{}", string)
    }
}
