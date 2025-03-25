use crate::terminal::{Position, Size, Terminal};
use crate::view::View;
use crossterm::event::{read, Event, KeyCode::{self, Char}, KeyEvent, KeyModifiers};
use core::cmp::min;
use std::{env, io::Error};

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct  Location {
    x: usize,
    y: usize,
}

impl Editor {

    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        
        Terminal::init()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(filename) = args.get(1) {
            view.load(filename);
        }
        Ok(Editor {
            should_quit: false,
            location: Location::default(),
            view
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.process_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event. Error: {err:?}")
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret(Position::new(self.location.y, self.location.x));
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn process_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code, 
                modifiers, 
                ..}) => {
                    match code {
                        Char('q') if modifiers == KeyModifiers::CONTROL => {
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
                            self.move_point(code);
                        }
                        _ => ()
                    }
                }
            Event::Resize(width_u16, height_u16 ) => {
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                self.view.resize(Size::new(width, height));
            }
            _ => ()
        }
    }

    fn move_point(&mut self, keycode: KeyCode) {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size().unwrap_or_default();
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
    }
    
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Thank you for using ed!");
        }
    }
}

