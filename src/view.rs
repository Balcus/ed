use crate::terminal::{Size, Terminal};
use std::io::Error;
use crate::buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION"); 

#[derive(Default, Debug)]
pub struct View {
    pub buffer: Buffer,
}

impl View {

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        }else {
            self.render_buffer()?;
        }
        Ok(())
    }

    pub fn render_welcome_screen() -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            
            if row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            Self::draw_empty_row()?;
            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("Welcome to {NAME} -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message.as_str())?;
        Ok(())
    }

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }
}