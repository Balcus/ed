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

    pub fn number_of_lines(&self) -> usize {
        self.lines.len()
    }

    // === Edit Buffer === //

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

    // === Save === //

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

    // === Search === //

    pub(crate) fn search_forward(&self, from: Location, query: &Line) -> Option<Location> {
        if query.is_empty() {
            return None;
        }

        let mut is_first = true;
        for (index, line) in self
            .lines
            .iter()
            .enumerate()
            .cycle()
            .skip(from.line_index)
            .take(self.lines.len().saturating_add(1))
        {
            let from_grapheme_index = if is_first {
                is_first = false;
                from.grapheme_index
            } else {
                0
            };

            if let Some(grapheme_index) = line.search_forward(from_grapheme_index, query) {
                return Some(Location {
                    grapheme_index,
                    line_index: index,
                });
            }
        }
        None
    }

    pub(crate) fn search_backwards(&self, from: Location, query: &Line) -> Option<Location> {
        if query.is_empty() {
            return None;
        }

        let mut is_first = true;
        for (index, line) in self
            .lines
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(
                self.lines
                    .len()
                    .saturating_sub(from.line_index)
                    .saturating_sub(1),
            )
            .take(self.lines.len().saturating_add(1))
        {
            let from_grapheme_index = if is_first {
                is_first = false;
                from.grapheme_index
            } else {
                line.grapheme_count()
            };

            if let Some(grapheme_index) = line.search_backward(from_grapheme_index, query) {
                return Some(Location {
                    grapheme_index,
                    line_index: index,
                });
            }
        }
        None
    }
}
