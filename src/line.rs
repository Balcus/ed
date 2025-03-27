use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Line {
    string: Vec<String>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: line_str.graphemes(true)
            .map(String::from)
            .collect(),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().join("")
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}