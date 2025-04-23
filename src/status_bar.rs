use crate::terminal::{Terminal, Size};
use crate::editor::DocumentStatus;

pub struct StatusBar {
    status: DocumentStatus,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: size.height.saturating_sub(margin_bottom).saturating_sub(1),
        }
    }

    pub fn resize(&mut self, size: Size) {
        todo!()
    }

    pub fn update_status(&mut self, new_status: DocumentStatus) {
        todo!()
    }

    pub fn render(&mut self) {
        todo!()
    }
}
