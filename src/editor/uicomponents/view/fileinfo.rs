use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

use super::FileType;

#[derive(Default, Debug)]
pub struct FileInfo {
    path: Option<PathBuf>,
    file_type: FileType,
}
impl FileInfo {
    pub fn from(file_name: &str) -> Self {
        let path_buf = PathBuf::from(file_name);
        let file_type = FileType::from(&path_buf);
        Self {
            path: Some(path_buf),
            file_type,
        }
    }
    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }

    pub fn get_file_type(&self) -> FileType {
        self.file_type
    }
}
impl Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(f, "{name}")
    }
}
