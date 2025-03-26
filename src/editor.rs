use crate::terminal::Terminal;
use crate::view::View;
use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event, KeyEvent};
use std::{env, io::Error};
use crate::editor_commands::Command;

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    view: View,
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
        let _ = Terminal::move_caret(self.view.get_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn process_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent {kind, ..}) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            match Command::try_from(event) {
                Ok(command) => {
                    if matches!(command, Command::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Command recived could not be handled: {e}")
                    }
                }
            }
        }
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

