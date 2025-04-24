use crate::document_status::DocumentStatus;
use crate::line::Line;
use crate::terminal::{Position, Size, Terminal};
use crate::buffer::Buffer;
use crate::editor_commands::{Command, Direction::{self, Up, Down, Left, Right, PageDown, PageUp, Home, End, WordJumpLeft, WordJumpRight}};
use std::cmp::min;

pub const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION"); 

#[derive(Default)]
pub struct Location {
    pub line_index: usize,
    pub grapheme_index: usize,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    bottom_margin: usize,
}

impl View {

    pub fn new(margin_bottom: usize) -> Self {
        let terminal_size = Terminal::size().unwrap_or_default();
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Size {
                height: terminal_size.height.saturating_sub(margin_bottom),
                width: terminal_size.width,
            },
            text_location: Location::default(),
            scroll_offset: Position::default(),
            bottom_margin: margin_bottom,
        }
    }

    // Rendering functions

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;
        let top = self.scroll_offset.row;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_substr(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }


    // Other important functions

    pub fn handle_command(&mut self, command: &Command) {
        match command {
            Command::Resize(size) => self.resize(*size),
            Command::Move(direction) => self.move_text_location(&direction),
            Command::Insert(c) => self.insert_character(*c),
            Command::Backspace => self.backspace(),
            Command::Delete => self.delete(),
            Command::Enter => self.insert_newline(),
            Command::RemoveLine => self.delete_line(),
            Command::Save => self.save_file(),
            Command::Quit => {},
        }
    }

    fn resize(&mut self, to: Size) {
        self.size = Size {
            height: to.height.saturating_sub(self.bottom_margin),
            width: to.width,
        };
        self.scroll_text_location_into_view();
        self.needs_redraw = true;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        
        let remainign_width = width.saturating_sub(1);

        if remainign_width < len {
            return "~".to_string();
        }
        
        format!("{:<1}{:^remainign_width$}", "~", welcome_message)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line|{
            line.sum_width_until(self.text_location.grapheme_index)
        });
        
        Position {
            row,
            col
        }
    }

    pub fn get_caret_position(&self) -> Position {
        self.text_location_to_position().saturating_sub(self.scroll_offset)
    }

    fn save_file(&mut self) {
        let _ = self.buffer.save();
    }

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            file_name: format!("{}", self.buffer.file_info),
            number_of_lines: self.buffer.number_of_lines(),
            line_number: self.text_location.line_index,
            modified: self.buffer.dirty,
        }
    }


    // Write text

    fn insert_character(&mut self, c: char) {
        let old_grapheme_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        self.buffer.insert_char(c, &self.text_location);

        let new_grapheme_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        
        let grapheme_difference = new_grapheme_len.saturating_sub(old_grapheme_len);

        if grapheme_difference > 0 {
            self.move_text_location(&Direction::Right);
        }

        self.needs_redraw = true;
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(&self.text_location);
        self.move_text_location(&Direction::Right);
        self.needs_redraw = true;
    }

    // Delete text

    fn backspace(&mut self) {
        if self.text_location.grapheme_index == 0 && self.text_location.line_index == 0 {
            return;
        }
        self.move_text_location(&Direction::Left);
        self.delete();
    }

    fn delete(&mut self) {
        self.buffer.delete(&self.text_location);
        self.needs_redraw = true;
    }

    fn delete_line(&mut self) {
        self.buffer.delete_line(self.text_location.line_index);
        self.move_up(1);
        self.needs_redraw = true;
    }
    

    // Movement functions 

    fn move_text_location(&mut self, direction: &Direction) {
        let Size { height, .. } = self.size;

        match direction {
            Up => self.move_up(1),
            Down => self.move_down(1),
            Left => self.move_left(),
            Right => self.move_right(),
            PageUp => self.move_up(height.saturating_sub(1)),
            PageDown => self.move_down(height.saturating_sub(1)),
            Home => self.move_to_beggining_of_line(),
            End => self.move_to_end_of_line(),
            WordJumpRight => self.jump_word_right(),
            WordJumpLeft => self.jump_word_left(),
        }

        self.scroll_text_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_right(&mut self) {
        let line_width = self.buffer.lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_beggining_of_line();
            self.move_down(1);
        }
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    fn move_to_beggining_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    /*
        Jump word functions will move the caret FROM inside a word TO the beggining of the next word (for right navigation)
        and to the last letter of the word before (for left navigation).
        They do not take into consideration any other separator such as: ',' , '.' , ':' , ... 
    */ 
    fn jump_word_right(&mut self) {
        if let Some(buffer_line) = self.buffer.lines.get(self.text_location.line_index) {
            let grapheme_count = buffer_line.grapheme_count();

            if self.text_location.grapheme_index >= grapheme_count {
                self.move_to_beggining_of_line();
                self.move_down(1);
                return;
            }

            let mut curr_index = self.text_location.grapheme_index;
            while curr_index < grapheme_count {
                if let Some(fragment) = buffer_line.get_fragments().get(curr_index) {
                    let is_white_space = fragment.grapheme.trim().is_empty() || 
                        fragment.replacement == Some('␣') ||
                        fragment.replacement == Some(' ');

                    if is_white_space {
                        curr_index = curr_index.saturating_add(1);
                        break;
                    }
                    
                    curr_index = curr_index.saturating_add(1);
                } else {
                    break;
                }
            }
            self.text_location.grapheme_index = curr_index;
        }
    }

    fn jump_word_left(&mut self) {
        if let Some(buffer_line) = self.buffer.lines.get(self.text_location.line_index) {

            if self.text_location.grapheme_index <= 0 {
                self.move_up(1);
                self.move_to_end_of_line();
                
                return;
            }

            let mut curr_index = self.text_location.grapheme_index;
            while curr_index > 0 {
                if let Some(fragment) = buffer_line.get_fragments().get(curr_index) {
                    let is_white_space = fragment.grapheme.trim().is_empty() || 
                        fragment.replacement == Some('␣') ||
                        fragment.replacement == Some(' ');

                    if is_white_space {
                        curr_index = curr_index.saturating_sub(1);
                        break;
                    }
                    
                    curr_index = curr_index.saturating_sub(1);
                } else {
                    break;
                }
            }
            self.text_location.grapheme_index = curr_index;
        }
    }


    // Scroll text

    fn scroll_horizontally(&mut self, to: usize) {
        let Size {width, ..} = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.needs_redraw = true;
        }
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size{height, ..} = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.needs_redraw = true;
        }
    }

    // Fixup functions

    fn scroll_text_location_into_view(&mut self) {
        let Position{ row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.number_of_lines())
    }
}