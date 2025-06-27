use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::size::Size;

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
            code, modifiers, ..
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
