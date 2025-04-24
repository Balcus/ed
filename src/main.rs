#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod view;
mod buffer;
mod editor_commands;
mod line;
use editor::Editor;
mod status_bar;
mod document_status;
mod file_info;
mod args;

fn main() {
    let mut ed = Editor::new().unwrap();
    if let Some(file_name) = args::parse_args() {
        ed.load(&file_name);
    }
    ed.init().unwrap();
    ed.run();
}
