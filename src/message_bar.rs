use crate::{terminal::{Size, Terminal}, ui_component::UiComponent};

#[derive(Default)]
pub struct MessageBar {
    message: String,
    needs_redraw: bool,
    size: Size,
}

impl MessageBar {
    pub fn change_message(&mut self, new_message: String) {
        self.message = new_message;
        self.mark_redraw(true);
    }
}

impl UiComponent for MessageBar {
    fn mark_redraw(&mut self, val: bool) {
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: crate::terminal::Size) {
        self.size = size;
        self.mark_redraw(true);
    }

    fn draw(&mut self, position_y: usize) -> Result<(), std::io::Error> {
        Terminal::print_row(position_y, &self.message)
    }
}