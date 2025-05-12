use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

/// Represents information about a file.
#[derive(Default, Debug)]
pub struct FileInfo {
    /// The path to the file.
    path: Option<PathBuf>,
}

impl FileInfo {
    /// Creates a new `FileInfo` instance from a file name.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file.
    ///
    /// # Returns
    ///
    /// A new `FileInfo` instance.
    pub fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name)),
        }
    }

    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }
}

impl Display for FileInfo {
    /// Formats the `FileInfo` for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(f, "{name}")
    }
}
