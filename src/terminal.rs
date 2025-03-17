use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};
use core::fmt::Display;

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}
#[derive(Copy, Clone)]
pub struct CursorPos {
    pub row: u16,
    pub col: u16,
}
pub struct Terminal;

impl Terminal {

    // run when closing editor
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    // run when first opening editor
    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear()?;
        Self::move_cursor(CursorPos { row: 0, col: 0 })?;
        Self::execute()?;
        Ok(())
    }

    // clears entire screen
    pub fn clear() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    // cleears a line
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor(position: CursorPos) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.row, position.col))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }

    // prints the output buffer
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
    
}