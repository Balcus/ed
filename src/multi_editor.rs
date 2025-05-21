use crate::editor_commands::{
    Command::System,
    System::{Quit, Resize},
};
use crate::{editor::Editor, editor_commands::Command, size::Size, terminal::Terminal};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use std::io::Error;
use std::vec;

pub struct MultiEditor {
    editors: Vec<Editor>,
    active_editor: usize,
    terminal_size: Size,
    should_quit: bool,
}

impl Default for MultiEditor {
    fn default() -> Self {
        Self {
            editors: vec![Editor::new()],
            active_editor: 0,
            terminal_size: Size::default(),
            should_quit: false,
        }
    }
}

impl MultiEditor {
    pub fn new() -> Self {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        let mut multi_editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        multi_editor.resize(size);
        multi_editor
    }

    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        for editor in &mut self.editors {
            editor.handle_resize_command(size);
        }
    }

    pub fn init() -> Result<(), Error> {
        Terminal::init()
    }

    fn active_editor(&mut self) -> &mut Editor {
        &mut self.editors[self.active_editor]
    }

    pub fn load(&mut self, file_names: Vec<String>) {
        if file_names.is_empty() {
            return;
        }

        self.active_editor().load(&file_names[0]);

        for file_name in file_names.iter().skip(1) {
            self.create_new_editor();
            self.active_editor += 1;
            self.active_editor().load(file_name);
        }
    }

    fn refresh_screen(&mut self) {
        self.active_editor().refresh_screen();
    }

    fn change_editor_message(&mut self, message: &str) {
        self.active_editor()
            .get_message_bar()
            .update_message(message);
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event. Error: {err:?}")
                    }
                }
            }
        }
    }

    fn evaluate_event(&mut self, event: Event) {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            ..
        }) = event
        {
            let editor_index = self.active_editor.saturating_add(1);
            self.switch_editor(editor_index);
            return;
        }

        if let Event::Key(KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            ..
        }) = event
        {
            let editor_index = self.active_editor.saturating_sub(1);
            self.switch_editor(editor_index);
            return;
        }

        if let Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            ..
        }) = event
        {
            self.create_new_editor();
            return;
        }

        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &crossterm::event::KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }

    fn switch_editor(&mut self, editor_index: usize) {
        if editor_index < self.editors.len() {
            self.active_editor = editor_index;
            let _ = Terminal::clear();
            self.active_editor().set_needs_redraw(true);
            self.refresh_screen();
            self.change_editor_message(&format!("Switched to editor window {editor_index}"));
        }
    }

    fn create_new_editor(&mut self) {
        self.editors.push(Editor::new());
        self.change_editor_message(&format!(
            "New editor created, {} editors are open",
            self.editors.iter().len()
        ));
    }

    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => {
                self.active_editor().process_command(command);

                if self.active_editor().should_quit {
                    if self.editors.len() <= 1 {
                        self.should_quit = true;
                    } else {
                        self.editors.remove(self.active_editor);
                        if self.active_editor >= self.editors.len() {
                            self.active_editor = self.editors.len().saturating_sub(1);
                        }
                        let _ = Terminal::clear();
                        self.active_editor().set_needs_redraw(true);
                    }
                }
            }
            System(Resize(size)) => {
                self.resize(size);
            }
            _ => self.active_editor().process_command(command),
        }
    }
}

impl Drop for MultiEditor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Thank you for using ed!");
        }
    }
}
