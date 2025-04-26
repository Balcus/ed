use crate::terminal::{Terminal, Size};
use crate::document_status::DocumentStatus;
use crate::ui_component::UiComponent;

#[derive(Default)]

pub struct StatusBar {
    status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if self.status != new_status {
            self.status = new_status;
            self.mark_redraw(true);
        }

    }
}

impl UiComponent for StatusBar {
    fn mark_redraw(&mut self, val: bool) {
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, position_y: usize) -> Result<(), std::io::Error> {
        if let Ok(size) = Terminal::size() {
            let line_count = self.status.line_count_to_string();
            let modified_indicator = self.status.modified_indicator_to_string();

            let beginning = format!(
                "{} - {line_count} {modified_indicator}",
                self.status.file_name
            );

            let position_indicator = self.status.position_indicator_to_string();
            let remainder_len = size.width.saturating_sub(beginning.len());
            let status = format!("{beginning}{position_indicator:>remainder_len$}");

            let to_print = if status.len() <= size.width {
                status
            } else {
                String::new()
            };

            let result = Terminal::print_inverted_row(position_y, &to_print);
            debug_assert!(result.is_ok(), "Failed to render status bar");
            self.mark_redraw(false);
        }
        Ok(())
    }
}
