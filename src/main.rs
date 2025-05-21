#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod editor;
mod terminal;
mod view;
mod buffer;
mod editor_commands;
mod line;
use multi_editor::MultiEditor;
mod status_bar;
mod document_status;
mod file_info;
mod args;
mod ui_component;
mod message_bar;
mod command_bar;
mod multi_editor;
mod position;
mod size;
mod serach_info;

/* TODO! :
    * file flag should work with multiple arguments, each opened in a separate editor window
    * add option to open a file from the command bar
    * continue with the find command
*/

fn main() {
    let mut ed = MultiEditor::new();
    if let Some(file_name) = args::parse_args() {
        ed.load(&file_name);
    }
    MultiEditor::init().unwrap();
    ed.run();
}

