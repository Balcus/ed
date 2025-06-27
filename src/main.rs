#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod command;
mod editor;
mod line;
mod terminal;
use multi_editor::MultiEditor;
mod args;
mod command_bar;
mod document_status;
mod message_bar;
mod multi_editor;
mod position;
mod size;
mod status_bar;
mod ui_component;
mod view;

fn main() {
    let mut ed = MultiEditor::new();
    let files = args::parse_args();
    ed.load(&files);
    MultiEditor::init().unwrap();
    ed.run();
}
