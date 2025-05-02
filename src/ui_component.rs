use std::io::Error;
use crate::size::Size;

pub trait UiComponent {
    fn mark_redraw(&mut self, val: bool);
    fn needs_redraw(&self) -> bool;
    fn set_size(&mut self, size: Size);
    fn draw(&mut self, position_y: usize) -> Result<(), Error>;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.mark_redraw(true);
    }

    fn render(&mut self, position_y: usize) {
        if self.needs_redraw() {
            match self.draw(position_y) {
                Ok(()) => self.mark_redraw(false),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {e:?}");
                    }
                },
            }
        }
    }
}