use std::time::{Duration, Instant};
use crate::{terminal::Terminal, ui_component::UiComponent};
use crate::size::Size;

const DEFAULT_DURATION: Duration = Duration::new(3, 0);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    message: Message,
    needs_redraw: bool,
    size: Size,
    message_cleared: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: &str) {
        self.message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.message_cleared = false;
        self.mark_redraw(true);
    }
    
    pub(crate) fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
}

impl UiComponent for MessageBar {
    fn mark_redraw(&mut self, val: bool) {
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool {
        (!self.message_cleared && self.message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.mark_redraw(true);
    }

    fn draw(&mut self, position_y: usize) -> Result<(), std::io::Error> {
        if self.message.is_expired() {
            self.message_cleared = true;
        }

        let message = if self.message.is_expired() {
            String::new()
        } else {
            self.message.text.to_string()
        };
        Terminal::print_row(position_y, &message)
    }
}