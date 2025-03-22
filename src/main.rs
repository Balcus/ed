#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod view;
use editor::Editor;

fn main() {
    let mut ed = Editor::default();
    ed.run();
}
