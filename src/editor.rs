use crate::terminal::{CursorPos, Size, Terminal};
use crossterm::event::{read, Event, KeyEvent, KeyModifiers, Event::Key, KeyCode::Char};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION"); 
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    
    // creates new editor obj
    pub fn new() -> Self {
        Editor{
            should_quit: false,
        }
    }

    // strats the editor
    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    // updates screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear()?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(CursorPos { row: (0), col: (0) })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    // updates screen and reads the event to be processed
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.process_event(&event);
        }
        Ok(())
    }

    // processes keyboard events
    fn process_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code, modifiers, kind: _, state: _ }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => ()
            }
        }
    }

    // draws each row
    fn draw_rows() -> Result<(), std::io::Error> {
        let Size {height, ..} = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            if row == height / 3 {
                Self::draw_welcome_message()?;
            }else {
                Self::draw_empty_row()?;
            }
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("Welcome to {NAME} editor -- version {VERSION} ⌨️");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message.as_str())?;
        Ok(())
    }
}

