use crate::line::Line;
use crate::ui_component::UiComponent;
use crate::terminal::Terminal;
use crate::size::Size;
use crate::editor_commands::Edit::{self, Insert, Delete, Backspace, Enter, RemoveLine};
use std::cmp::min;
use std::io::Error;

#[derive(Default)]
pub struct CommandBar {
    prompt: String,
    value: Line,
    needs_redraw: bool,
    size: Size,
}

impl UiComponent for CommandBar {
    fn mark_redraw(&mut self, val: bool) {
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, position_y: usize) -> Result<(), Error> {
        let value_area = self.size.width.saturating_sub(self.prompt.len());
        let value_end = self.value.width();
        let value_start = value_end.saturating_sub(value_area);

        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );

        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };

        self.mark_redraw(false);
        Terminal::print_row(position_y, &to_print)
    }
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Insert(c) =>  {
                self.value.append_char(c);
                self.mark_redraw(true);
            }
            Backspace => {
                self.value.delete_last();
                self.mark_redraw(true);
            }
            Delete | Enter | RemoveLine => {}
        }
    }

    pub fn caret_position_col(&self) -> usize {
        let max_width = self.prompt.len().saturating_add(self.value.grapheme_count());
        min(max_width, self.size.width)
    }

    pub fn value(&self) -> String {
        self.value.to_string()
    }

    pub fn set_prompt(&mut self, new_prompt: &str) {
        self.prompt = new_prompt.to_string();
        self.mark_redraw(true);
    }

    pub fn clear_value(&mut self) {
        self.value = Line::default();
        self.mark_redraw(true);
    }
}