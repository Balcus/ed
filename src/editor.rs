use crate::terminal::{Position, Size, Terminal};
use crate::view::View;
use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers};
use core::cmp::min;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

#[derive(Copy, Clone, Default)]
pub struct  Location {
    x: usize,
    y: usize,
}

impl Editor {

    // strats the editor
    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    // updates screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret(Position::default())?;
        if self.should_quit {
            Terminal::clear()?;
        } else {
            View::render()?;
            Terminal::move_caret(Position {
                row: (self.location.y),
                col: (self.location.x) 
            })?;
        }
        Terminal::show_caret()?;
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
            self.process_event(&event)?;
        }
        Ok(())
    }

    // processes keyboard events
    fn process_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent { 
            code, 
            modifiers, 
            kind: KeyEventKind::Press, 
            state: _ }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_point(*code)?;
                }
                _ => ()
            }
        }
        Ok(())
    }

    fn move_point(&mut self, keycode: KeyCode) -> Result<(), std::io::Error> {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
        match keycode {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
            },
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location{x,y};
        Ok(())
    }
    
}

