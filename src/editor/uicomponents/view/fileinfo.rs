//! File information management for the text editor.
//!
//! This module provides the `FileInfo` struct, which manages information
//! about files associated with text buffers. It handles file paths and
//! provides utilities for displaying file names in the editor interface.
//!
//! # Examples
//!
//! ```rust
//! use hecto::editor::uicomponents::view::fileinfo::FileInfo;
//!
//! // Create file info from a path
//! let file_info = FileInfo::from("/path/to/example.txt");
//! println!("File: {}", file_info); // Displays: "example.txt"
//!
//! // Check if a path is associated
//! assert!(file_info.has_path());
//! ```

use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

/// Contains information about a file associated with a text buffer.
///
/// `FileInfo` manages the file path and provides utilities for working with
/// file information in the editor. It can represent both files that exist
/// on disk and new, unsaved buffers.
///
/// # Fields
///
/// - `path`: Optional path to the file on disk
///
/// # Examples
///
/// ```rust
/// use hecto::editor::uicomponents::view::fileinfo::FileInfo;
///
/// // Create file info for an existing file
/// let file_info = FileInfo::from("example.txt");
///
/// // Create empty file info for a new buffer
/// let new_buffer = FileInfo::default();
/// assert!(!new_buffer.has_path());
/// ```
#[derive(Default, Debug)]
pub struct FileInfo {
    /// Optional path to the file on disk
    path: Option<PathBuf>,
}
impl FileInfo {
    /// Creates a new `FileInfo` instance from a file path.
    ///
    /// Constructs a `FileInfo` with the specified file path. The path is
    /// stored as a `PathBuf` for efficient manipulation and querying.
    ///
    /// # Parameters
    ///
    /// * `file_name` - The path to the file
    ///
    /// # Returns
    ///
    /// A new `FileInfo` instance with the specified path
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::fileinfo::FileInfo;
    ///
    /// let file_info = FileInfo::from("example.txt");
    /// assert!(file_info.has_path());
    /// ```
    pub fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name)),
        }
    }

    /// Gets a reference to the file path, if one exists.
    ///
    /// Returns an optional reference to the path as a `Path`. This is useful
    /// for performing path operations without taking ownership.
    ///
    /// # Returns
    ///
    /// `Some(&Path)` if a path is associated, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::fileinfo::FileInfo;
    ///
    /// let file_info = FileInfo::from("/path/to/example.txt");
    /// if let Some(path) = file_info.get_path() {
    ///     println!("File path: {}", path.display());
    /// }
    /// ```
    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Checks whether a file path is associated with this info.
    ///
    /// # Returns
    ///
    /// `true` if a path is associated, `false` for new/unsaved buffers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::fileinfo::FileInfo;
    ///
    /// let file_info = FileInfo::from("example.txt");
    /// assert!(file_info.has_path());
    ///
    /// let new_buffer = FileInfo::default();
    /// assert!(!new_buffer.has_path());
    /// ```
    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }
}
/// Implementation of the `Display` trait for showing file names.
///
/// This implementation formats the file info for display in the editor interface,
/// showing just the file name (without path) for associated files, or "[No Name]"
/// for new/unsaved buffers.
impl Display for FileInfo {
    /// Formats the file info for display.
    ///
    /// # Returns
    ///
    /// - The file name (without path) if a file is associated
    /// - "[No Name]" if no file is associated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hecto::editor::uicomponents::view::fileinfo::FileInfo;
    ///
    /// let file_info = FileInfo::from("/path/to/example.txt");
    /// assert_eq!(format!("{}", file_info), "example.txt");
    ///
    /// let new_buffer = FileInfo::default();
    /// assert_eq!(format!("{}", new_buffer), "[No Name]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(f, "{name}")
    }
}
