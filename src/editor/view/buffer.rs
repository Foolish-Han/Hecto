use std::{fs::read_to_string, io::Error};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let content = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in content.lines() {
            lines.push(String::from(line));
        }
        Ok(Self { lines })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
