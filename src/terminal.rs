use std::io::{stdout, Write};
use crossterm::{cursor::{Hide, Show}, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

pub struct Size {
    pub height: u16,
    pub _width: u16,
}

pub struct _CursorPos {
    row: u16,
    col: u16,
}
pub struct Terminal {}

impl Terminal {

    pub fn init() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear()?;
        Self::move_cursor(0, 0)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::move_cursor(0, 0)?;
        println!("Thank you for using ed!");
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (w, h) = crossterm::terminal::size()?;
        Ok(Size {
            height: h,
            _width: w,
        })
    }

    pub fn move_cursor(row: u16, col:u16) -> Result<(), std::io::Error> {
        queue!(stdout(), crossterm::cursor::MoveTo(col, row))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

}