use crate::message_bar::MessageBar;
use crate::status_bar::StatusBar;
use crate::terminal::{Size, Terminal};
use crate::ui_component::UiComponent;
use crate::view::View;
use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event, KeyEvent};
use std::io::Error;
use crate::editor_commands::Command;
use crate::view::NAME;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    title: String,
    terminal_size: Size,
    status_bar: StatusBar,
    message_bar: MessageBar,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);
        editor.refresh_status();
        editor.message_bar.update_message(String::from("HELP: ^S - save | ^Q - quit | ^L - line numbers"));
        Ok(editor)
    }

    pub fn init(&mut self) -> Result<(), Error> {
        Terminal::init()?;
        Ok(())
    }

    pub fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
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
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_caret();
        self.message_bar.render(self.terminal_size.height.saturating_sub(1));
        if self.terminal_size.height > 1 {
            self.status_bar.render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

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
                } else if let Command::Resize(size) = command {
                    self.resize(size);
                }else {
                    self.view.handle_command(&command);
                    if let Command::Resize(size) = command {
                        self.status_bar.resize(size);
                    }
                }
            }
        }
    }
    
    pub(crate) fn load(&mut self, file_name: &str) {
        self.view.load(file_name);
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

