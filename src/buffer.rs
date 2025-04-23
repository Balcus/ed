use std::fs::File;
use std::io::Write;
use std::{char, fs::read_to_string, io::Error};
use crate::line::Line;
use crate::view::Location;

#[derive(Default)]
pub struct Buffer {
    pub file_name: Option<String>,
    pub lines: Vec<Line>
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load(file_name: &str) -> Result<Self, Error> {
        let file_content = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in file_content.lines() {
            lines.push(Line::from(line));
        }
        
        Ok(Self {
            lines,
            file_name: Some(file_name.to_string()),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(name) = &self.file_name {
            let mut file = File::create(name)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }

    pub fn number_of_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, character: char, at: &Location) {
        if at.line_index > self.lines.len() {
            return;
        }

        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
        }
    }

    pub fn delete(&mut self, at: &Location) {
        if at.line_index >= self.lines.len() {
            return;
        }
        
        if at.grapheme_index >= self.lines[at.line_index].grapheme_count() {
            if at.line_index < self.lines.len() - 1 {
                let next_line = self.lines.remove(at.line_index + 1);
                self.lines[at.line_index].append(&next_line);
            }
        } else {
            self.lines[at.line_index].delete(at.grapheme_index);
        }
    }

    #[allow(dead_code)]
    pub fn delete_line(&mut self, at: usize) {
        if self.number_of_lines() > at {
            self.lines.remove(at);
        }
    }
    
    pub fn insert_newline(&mut self, at: &Location) {
        if at.line_index == self.number_of_lines() {
            self.lines.push(Line::default())
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
        }
    }
}