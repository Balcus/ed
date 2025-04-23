#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod view;
mod buffer;
mod editor_commands;
mod line;
use editor::Editor;
mod status_bar;

fn main() {
    let mut ed = Editor::new().unwrap();
    ed.run();
}
