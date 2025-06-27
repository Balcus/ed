use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
            code, modifiers, ..
        } = event;

        match (code, modifiers) {
            (KeyCode::Char('x'), KeyModifiers::CONTROL) => Ok(Self::RemoveLine),
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(c)),
            (KeyCode::Delete, _) => Ok(Self::Delete),
            (KeyCode::Backspace, _) => Ok(Self::Backspace),
            (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
            (KeyCode::Enter, _) => Ok(Self::Enter),
            _ => Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            )),
        }
    }
}
