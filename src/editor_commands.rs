use crate::terminal::Size;
use crossterm::event::{Event, KeyCode::{self, Char}, KeyModifiers, KeyEvent};

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
                    match code {
                        Char('q') if modifiers == KeyModifiers::CONTROL => {
                            return Ok(Self::Quit);
                        },
                        KeyCode::Right if modifiers == KeyModifiers::CONTROL => {
                            return Ok(Self::Move(Direction::WordJumpRight));
                        }
                        KeyCode::Left if modifiers == KeyModifiers::CONTROL => {
                            return Ok(Self::Move(Direction::WordJumpLeft));
                        }
                        KeyCode::Up => {
                            return Ok(Self::Move(Direction::Up));
                        }
                        KeyCode::Down => {
                            return Ok(Self::Move(Direction::Down));
                        }
                        KeyCode::Left => {
                            return Ok(Self::Move(Direction::Left));
                        }
                        KeyCode::Right => {
                            return Ok(Self::Move(Direction::Right));
                        }
                        KeyCode::PageDown => {
                            return Ok(Self::Move(Direction::PageDown));
                        }
                        KeyCode::PageUp => {
                            return Ok(Self::Move(Direction::PageUp));
                        }
                        KeyCode::Home => {
                            return Ok(Self::Move(Direction::Home));
                        }
                        KeyCode::End => {
                            return Ok(Self::Move(Direction::End));
                        }
                        _ => return Err(format!("The given KeyCode is currently not supported. Recived input: {code:?}")),
                    };
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

