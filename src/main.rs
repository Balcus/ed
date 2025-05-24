#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod buffer;
mod editor;
mod editor_commands;
mod line;
mod terminal;
mod view;
use multi_editor::MultiEditor;
mod args;
mod command_bar;
mod document_status;
mod file_info;
mod location;
mod message_bar;
mod multi_editor;
mod position;
mod serach_info;
mod size;
mod status_bar;
mod ui_component;

/* TODO! :
 * add option to open a file from the command bar
 * some visual feedback for find
*/

fn main() {
    let mut ed = MultiEditor::new();
    let files = args::parse_args();
    ed.load(&files);
    MultiEditor::init().unwrap();
    ed.run();
}
