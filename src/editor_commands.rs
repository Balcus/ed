use crate::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
    WordJumpRight,
    WordJumpLeft,
}

pub enum Command {
    Move(Direction),
    Resize(Size),
    Insert(char),
    Delete,
    Backspace,
    Quit,
}

impl TryFrom<Event> for Command {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, 
                modifiers, 
                ..}) => {
                    match (code, modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                        (KeyCode::Right, KeyModifiers::CONTROL) => Ok(Self::Move(Direction::WordJumpRight)),
                        (KeyCode::Left, KeyModifiers::CONTROL) => Ok(Self::Move(Direction::WordJumpLeft)),
                        (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                        (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                        (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                        (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                        (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                        (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                        (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                        (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                        (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(c)),
                        (KeyCode::Delete, _) => Ok(Self::Delete),
                        (KeyCode::Backspace, _) => Ok(Self::Backspace),
                        _ => Err(format!("Key Code is not supported: {code:?}")),
                    }
                },
                Event::Resize(width_u16, height_u16) => {
                    let width = width_u16 as usize;
                    let height = height_u16 as usize;
                    return Ok(Self::Resize(Size::new(width, height)))
                },
                _ => return Err(format!("The given event is currently not supported. Recived event: {event:?}")),
        }
    }
}

