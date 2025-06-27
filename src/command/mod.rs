use crossterm::event::Event;
use std::convert::TryFrom;
mod move_command;
pub use move_command::Move;
mod system;
pub use system::System;
mod edit;
pub use edit::Edit;
mod multi_editor;
pub use multi_editor::MultiEditorEvent;

use crate::size::Size;

#[derive(Copy, Clone)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
    MultiEditorCommand(MultiEditorEvent),
}

impl TryFrom<Event> for Command {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => MultiEditorEvent::try_from(key_event)
                .map(Command::MultiEditorCommand)
                .or_else(|_| Edit::try_from(key_event).map(Command::Edit))
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                width: width_u16 as usize,
                height: height_u16 as usize,
            }))),
            _ => Err(format!(
                "The given event is currently not supported. Recived event: {event:?}"
            )),
        }
    }
}
