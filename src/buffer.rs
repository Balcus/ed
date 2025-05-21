use crate::file_info::FileInfo;
use crate::line::Line;
use crate::location::Location;
use std::fs::File;
use std::io::Write;
use std::{char, fs::read_to_string, io::Error};

#[derive(Default)]
pub struct Buffer {
    pub file_info: FileInfo,
    pub lines: Vec<Line>,
    pub dirty: bool,
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
            file_info: FileInfo::from(file_name),
            dirty: true,
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(name) = &self.file_info.path {
            let mut path = File::create(name)?;
            for line in &self.lines {
                writeln!(path, "{line}")?;
            }
            self.dirty = false;
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
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
            self.dirty = true;
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
                self.dirty = true;
            }
        } else {
            self.lines[at.line_index].delete(at.grapheme_index);
            self.dirty = true;
        }
    }

    #[allow(dead_code)]
    pub fn delete_line(&mut self, at: usize) {
        if self.number_of_lines() > at {
            self.lines.remove(at);
            self.dirty = true;
        }
    }

    pub fn insert_newline(&mut self, at: &Location) {
        if at.line_index == self.number_of_lines() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
            self.dirty = true;
        }
    }

    pub(crate) fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(file_name);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.dirty = false;
        Ok(())
    }

    fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(file_path) = &file_info.get_path() {
            let mut file = File::create(file_path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }

    pub(crate) fn search(&self, from: &Location, query: &str) -> Option<Location> {
        for (line_index, line) in self.lines.iter().enumerate().skip(from.line_index) {
            let from_grapheme_index = if line_index == from.line_index {
                from.grapheme_index
            } else {
                0
            };
            if let Some(grapheme_index) = line.search(from_grapheme_index, query) {
                return Some(Location {
                    line_index,
                    grapheme_index,
                });
            }
        }
        for (line_index, line) in self.lines.iter().enumerate() {
            if line_index <= from.line_index {
                if let Some(grapheme_index) = line.search(0, query) {
                    return Some(Location {
                        line_index,
                        grapheme_index,
                    });
                }
            }
        }
        None
    }
}
