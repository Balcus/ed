#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
use editor::Editor;

fn main() {
    let mut ed = Editor::default();
    ed.run();
}
