use std::{fs::read_to_string, io::Error};
use crate::line::Line;
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let file_content = read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in file_content.lines() {
            lines.push(Line::from(line));
        }
        
        Ok(Self {
            lines,
        })
    }

    pub fn number_of_lines(&self) -> usize {
        self.lines.len()
    }
}