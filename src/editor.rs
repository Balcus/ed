use crate::message_bar::MessageBar;
use crate::status_bar::StatusBar;
use crate::terminal::{Position, Size, Terminal};
use crate::ui_component::UiComponent;
use crate::view::View;
use crossterm::event::KeyEventKind;
use crossterm::event::{read, Event, KeyEvent};
use std::io::Error;
use crate::editor_commands::{
    Command::{self, Edit, Move, System},
    System::{Quit, Resize, Save, ShowLineNumbers, Dismiss},
    Edit::Enter,
};
use crate::view::NAME;
use crate::command_bar::CommandBar;

const TIMES_FOR_QUIT: u8 = 2;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    title: String,
    terminal_size: Size,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: Option<CommandBar>,
    quit_times: u8,
}

impl Editor {
    pub fn new() -> Self {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);
        editor.refresh_status();
        editor.message_bar.update_message("HELP: ^S - save | ^Q - quit | ^L - line");
        editor
    }

    pub fn init() -> Result<(), Error> {
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
        if let Some(command_bar) = &mut self.command_bar {
            command_bar.resize(Size {
                height: 1,
                width: size.width,
            });
        }
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
                Ok(event) => self.evaluate_event(event),
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

        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);

        let _ = Terminal::hide_caret();

        if let Some(command_bar) = &mut self.command_bar {
            command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }

        if self.terminal_size.height > 1 {
            self.status_bar.render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        let new_caret_position = if let Some(command_bar) = &mut self.command_bar {
            Position {
                row: bottom_bar_row,
                col: command_bar.caret_position_col(),
            }
        } else {
            self.view.get_caret_position()
        };

        let _ = Terminal::move_caret(new_caret_position);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent {kind, ..}) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };


        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }
    
    pub(crate) fn load(&mut self, file_name: &str) {
        if self.view.load(file_name).is_err() {
            self.message_bar.update_message(&format!("ERROR: Failed to read file {file_name}"));
        }
    }
    
    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => if self.command_bar.is_none() {
                self.handle_quit();
            },
            System(Resize(size)) => self.resize(size),
            _ => self.quit_times = 0,
        }

        match command {
            System(Quit | Resize(_)) => {},
            System(Save) => {
                if self.command_bar.is_none() {
                    self.handle_save();
                }
            },
            System(ShowLineNumbers) => {
                self.view.change_line_numbers();
            },
            System(Dismiss) => {
                if self.command_bar.is_some() {
                    self.dismiss_prompt();
                    self.message_bar.update_message("Save aborted");
                }
            },
            Edit(editor_command) => {
                if let Some(command_bar) = &mut self.command_bar {
                    if matches!(editor_command, Enter) {
                        let file_name = command_bar.value();
                        self.dismiss_prompt();
                        self.save(Some(&file_name));
                    } else {
                        command_bar.handle_edit_command(editor_command);
                    }
                } else {
                    self.view.handle_edit_command(editor_command);
                }
            },
            Move(move_command)=>  {
                if self.command_bar.is_none() {
                    self.view.handle_move_command(move_command);
                }
            },
        }
    }
    
    fn handle_quit(&mut self) {
        if !self.view.get_status().modified || self.quit_times + 1 == TIMES_FOR_QUIT {
            self.should_quit = true;
        } else {
            self.message_bar.update_message(&format!(
                "WARNING: File has unsaved changes. Press ^Q {} more times to exit",
                TIMES_FOR_QUIT - self.quit_times - 1
            ));
        }
        self.quit_times += 1;
    }
    
    fn dismiss_prompt(&mut self) {
        self.command_bar = None;
        self.message_bar.mark_redraw(true);
    }
    
    fn save(&mut self, file_name: Option<&String>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };

        if result.is_err() {
            self.message_bar.update_message("Failed to save file");
        } else {
            self.message_bar.update_message("File saved successfully");
        }
    }

    fn handle_save(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.show_prompt();
        }
    }

    fn show_prompt(&mut self) {
        let mut command_bar = CommandBar::default();
        command_bar.set_prompt("Save as: ");
        command_bar.resize(Size {
            height: 1,
            width: self.terminal_size.width,
        });
        command_bar.mark_redraw(true);
        self.command_bar = Some(command_bar);
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

