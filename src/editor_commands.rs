use crate::size::Size;
use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};

#[derive(Copy, Clone)]
pub enum Move {
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

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, String> {
        let KeyEvent {
            code,
            modifiers,
            ..
        } = event;
        
        match (code, modifiers) {
            (KeyCode::Right, KeyModifiers::CONTROL) => Ok(Self::WordJumpRight),
            (KeyCode::Left, KeyModifiers::CONTROL) => Ok(Self::WordJumpLeft),
            (KeyCode::Up, _) => Ok(Self::Up),
            (KeyCode::Down, _) => Ok(Self::Down),
            (KeyCode::Left, _) => Ok(Self::Left),
            (KeyCode::Right, _) => Ok(Self::Right),
            (KeyCode::PageDown, _) => Ok(Self::PageDown),
            (KeyCode::PageUp, _) => Ok(Self::PageUp),
            (KeyCode::Home, _) => Ok(Self::Home),
            (KeyCode::End, _) => Ok(Self::End),
            _ => Err(format!("Unsupported key code {code:?} or modifier {modifiers:?}"))
        }
    }
}

#[derive(Copy, Clone)]
pub enum Edit {
    Insert(char),
    Delete,
    Enter,
    Backspace,
    RemoveLine,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code,
            modifiers,
            ..
        } = event;

        match (code, modifiers) {
            (KeyCode::Char('x'), KeyModifiers::CONTROL) => Ok(Self::RemoveLine),
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(c)),
            (KeyCode::Delete, _) => Ok(Self::Delete),
            (KeyCode::Backspace, _) => Ok(Self::Backspace),
            (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
            (KeyCode::Enter, _) => Ok(Self::Enter),
            _ => Err(format!("Unsupported key code {code:?} or modifier {modifiers:?}"))
        }
    }
}

#[derive(Copy, Clone)]
pub enum System {
    Save,
    Quit,
    Resize(Size),
    ShowLineNumbers,
    Dismiss,
    Search,
}


impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code,
            modifiers,
            ..
        } = event;

        match (code, modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => Ok(Self::ShowLineNumbers),
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => Ok(Self::Search),
            (KeyCode::Esc, _) => Ok(Self::Dismiss),
            _ => Err(format!("Key Code is not supported: {code:?}")),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
                Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                    width: width_u16 as usize,
                    height: height_u16 as usize,
                }))),
                _ => Err(format!("The given event is currently not supported. Recived event: {event:?}")),
        }
    }
}

