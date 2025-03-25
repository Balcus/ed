#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod view;
mod buffer;
use editor::Editor;

fn main() {
    let mut ed = Editor::new().unwrap();
    ed.run();
}
