use crate::line::Line;
use crate::terminal::{Size, Terminal, Position};
use crate::buffer::Buffer;
use crate::editor_commands::{Command, Direction::{self, Up, Down, Left, Right, PageDown, PageUp, Home, End, WordJumpLeft, WordJumpRight}};
use std::cmp::min;

const NAME: &str = env!("CARGO_PKG_NAME");
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
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}

impl View {

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
                Self::render_line(current_row, &line.get(left..right));
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

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Resize(size) => self.resize(size),
            Command::Move(direction) => self.move_text_location(&direction),
            Command::Quit => {}
        }
    }

    fn resize(&mut self, to: Size) {
        self.size = to;
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
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }


        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
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


    // Movement functions 

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
            WordJumpRight => {},
            WordJumpLeft => {},
        }

        self.scroll_text_location_into_view();
    }


    // Scroll text
    
    fn scroll_text_location_into_view(&mut self) {
        let Position{ row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

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

        self.needs_redraw = self.needs_redraw || offset_changed;
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

        self.needs_redraw = self.needs_redraw || offset_changed;
    }


    // Fixup functions

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