use std::io::stdout;
use crossterm::{cursor::Hide, cursor::Show, queue, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

pub struct Size {
    height: u16,
    width: u16,
}

pub struct Cursor_pos {
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

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        crossterm::terminal::size()
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

}