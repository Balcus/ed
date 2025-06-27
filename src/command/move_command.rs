use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
            code, modifiers, ..
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
            _ => Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            )),
        }
    }
}
