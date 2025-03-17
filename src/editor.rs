use crate::terminal::Terminal;
use crossterm::event::{read, Event, KeyEvent, KeyModifiers, Event::Key, KeyCode::Char};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    
    pub fn new() -> Self {
        Editor{
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        Terminal::init().unwrap();
        Self::draw_rows().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear()?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(0, 0)?;
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.process_event(&event);
            Terminal::hide_cursor()?;
            self.refresh_screen()?;
            Terminal::execute()?;
            Terminal::show_cursor()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

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

    fn draw_rows() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        let height = size.height;
        for row in 0..height {
            Terminal::move_cursor(row, 0)?;
            Terminal::print("~")?;
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}

