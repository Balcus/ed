use crate::command_bar::CommandBar;
use crate::editor_commands::{
    Command::{self, Edit, Move, System},
    Edit::Enter,
    Move::{Down, Up},
    System::{Dismiss, Quit, Resize, Save, Search, ShowLineNumbers},
};
use crate::message_bar::MessageBar;
use crate::position::Position;
use crate::size::Size;
use crate::status_bar::StatusBar;
use crate::terminal::Terminal;
use crate::ui_component::UiComponent;
use crate::view::{NAME, View};

const TIMES_FOR_QUIT: u8 = 2;

#[derive(PartialEq, Eq, Default)]
enum PromptType {
    Search,
    Save,
    #[default]
    None,
}

impl PromptType {
    fn is_none(&self) -> bool {
        *self == Self::None
    }
}

#[derive(Default)]
pub struct Editor {
    pub should_quit: bool,
    view: View,
    title: String,
    terminal_size: Size,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: CommandBar,
    prompt_type: PromptType,
    quit_times: u8,
}

impl Editor {
    pub fn new() -> Self {
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor.refresh_status();
        editor
            .message_bar
            .update_message("help: ^S - save | ^Q - quit | ^F find (see help page for more)");
        editor
    }

    fn in_prompt(&self) -> bool {
        !self.prompt_type.is_none()
    }

    pub fn set_needs_redraw(&mut self, val: bool) {
        self.view.mark_redraw(val);
        self.message_bar.mark_redraw(val);
        self.status_bar.mark_redraw(val);
    }

    pub const fn get_message_bar(&mut self) -> &mut MessageBar {
        &mut self.message_bar
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    pub fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);

        let _ = Terminal::hide_caret();

        match self.prompt_type {
            PromptType::None => self.message_bar.render(bottom_bar_row),
            PromptType::Save | PromptType::Search => self.command_bar.render(bottom_bar_row),
        }

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
            self.refresh_status();
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        let new_caret_position = if self.in_prompt() {
            Position {
                row: bottom_bar_row,
                col: self.command_bar.caret_position_col(),
            }
        } else {
            self.view.get_caret_position()
        };

        let _ = Terminal::move_caret(new_caret_position);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    pub(crate) fn load(&mut self, file_name: &str) {
        if self.view.load(file_name).is_err() {
            self.message_bar
                .update_message(&format!("ERROR: Failed to read file {file_name}"));
        } else {
            self.refresh_status();
        }
    }

    fn process_command_no_prompt(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.handle_quit();
            return;
        }
        self.reset_quit_times();

        match command {
            System(Quit | Resize(_) | Dismiss) => {}
            System(Search) => self.set_prompt(PromptType::Search),
            System(Save) => self.handle_save_command(),
            System(ShowLineNumbers) => self.toggle_line_numbers(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
        }
    }

    fn process_command_during_save(&mut self, command: Command) {
        match command {
            System(Quit | Resize(_) | Search | Save | ShowLineNumbers) | Move(_) => {}
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.message_bar.update_message("Save aborted!");
            }
            Edit(Enter) => {
                let file_name = self.command_bar.value();
                self.save(Some(&file_name));
                self.set_prompt(PromptType::None);
            }
            Edit(edit_command) => self.command_bar.handle_edit_command(edit_command),
        }
    }

    fn process_command_during_search(&mut self, command: Command) {
        match command {
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.view.dimiss_search();
            }
            Edit(Enter) => {
                self.set_prompt(PromptType::None);
                self.view.exit_search();
            }
            Edit(edit_command) => {
                self.command_bar.handle_edit_command(edit_command);
                let query = self.command_bar.value();
                self.view.search(&query);
            }
            Move(Down) => self.view.search_next(),
            Move(Up) => self.view.search_prev(),
            Move(_) | System(Quit | Resize(_) | Search | Save | ShowLineNumbers) => {}
        }
    }

    pub fn process_command(&mut self, command: Command) {
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);
            return;
        }

        match self.prompt_type {
            PromptType::None => self.process_command_no_prompt(command),
            PromptType::Save => self.process_command_during_save(command),
            PromptType::Search => self.process_command_during_search(command),
        }
    }

    pub fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        let bar_size = Size {
            height: 1,
            width: size.width,
        };
        self.message_bar.resize(bar_size);
        self.status_bar.resize(bar_size);
        self.command_bar.resize(bar_size);
    }

    pub fn handle_quit(&mut self) {
        if !self.view.get_status().modified || self.quit_times + 1 == TIMES_FOR_QUIT {
            self.should_quit = true;
        } else {
            self.message_bar.update_message(&format!(
                "WARNING: File has unsaved changes. Press ^Q {} more times to exit",
                TIMES_FOR_QUIT - self.quit_times - 1
            ));
            self.quit_times += 1;
        }
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
            self.refresh_status();
        }
    }

    fn handle_save_command(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.set_prompt(PromptType::Save);
        }
    }

    pub fn toggle_line_numbers(&mut self) {
        self.view.toggle_line_numbers();
    }

    const fn reset_quit_times(&mut self) {
        self.quit_times = 0;
    }

    fn set_prompt(&mut self, prompt_type: PromptType) {
        match prompt_type {
            PromptType::None => self.message_bar.set_needs_redraw(true),
            PromptType::Save => self.command_bar.set_prompt("Save as: "),
            PromptType::Search => {
                self.view.enter_search();
                self.command_bar.set_prompt("Find: ");
            }
        }
        self.command_bar.clear_value();
        self.prompt_type = prompt_type;
    }
}
