use crate::terminal::{Position, Size, Terminal};
use std::io::Error;
use crate::buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION"); 

#[derive(Debug)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(())
        }
        
        let Size{height, width} = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for curr_line in 0..height {
            if let Some(line) = self.buffer.lines.get(curr_line) {
                let displayed_line;
                if line.len() >= width {
                    displayed_line = &line[0..width]
                }else {
                    displayed_line = line;
                }
                Self::render_line(curr_line, displayed_line)?;
            }else if curr_line == vertical_center && self.buffer.is_empty() {
                Self::render_line(curr_line, Self::build_welcome_message(width).as_str())?;
            }else {
                Self::render_line(curr_line, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    pub fn render_line(line_number: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret(Position {
            row: line_number,
            col: 0
        })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())

    }

    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::from("");
        }

        let welcome_message = format!("Welcome to {NAME} - version {VERSION}");
        if welcome_message.len() >= width {
            return String::from("~");
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(welcome_message.len()).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }

    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.needs_redraw = true;
    }
}