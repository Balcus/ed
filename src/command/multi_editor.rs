use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Copy, Clone)]
pub enum MultiEditorEvent {
    NewTab,
    LeftTab,
    RightTab,
    Open,
}

impl TryFrom<KeyEvent> for MultiEditorEvent {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, String> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        match (code, modifiers) {
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => Ok(Self::NewTab),
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => Ok(Self::RightTab),
            (KeyCode::Char('j'), KeyModifiers::CONTROL) => Ok(Self::LeftTab),
            (KeyCode::Char('o'), KeyModifiers::CONTROL) => Ok(Self::Open),
            _ => Err(format!("Key Code is not supported: {code:?}")),
        }
    }
}
