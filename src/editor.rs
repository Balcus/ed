use crate::status_bar::StatusBar;
use crate::terminal::Terminal;
use crate::view::View;
use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event, KeyEvent};
use std::{env, io::Error};
use crate::editor_commands::Command;
use crate::view::NAME;

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    title: String,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        
        Terminal::init()?;
        
        let mut editor = Self {
            should_quit: false,
            view: View::new(2),
            status_bar: StatusBar::new(1),
            title: String::new(),
        };

        let args: Vec<String> = env::args().collect();

        if let Some(filename) = args.get(1) {
            editor.view.load(filename);
        }

        editor.refresh_status();
        Ok(editor)
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    pub fn run(&mut self) {
        loop {
            let status = self.view.get_status();
            self.status_bar.update_status(status);
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
        self.status_bar.render();
        let _ = Terminal::move_caret(self.view.get_caret_position());
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
            if let Ok(command) = Command::try_from(event) {
                if matches!(command, Command::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(&command);
                    if let Command::Resize(size) = command {
                        self.status_bar.resize(size);
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

