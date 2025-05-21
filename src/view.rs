use crate::document_status::DocumentStatus;
use crate::line::Line;
use crate::serach_info::SearchInfo;
use crate::terminal::Terminal;
use crate::size::Size;
use crate::buffer::Buffer;
use crate::editor_commands::{Edit, Move};
use crate::ui_component::UiComponent;
use std::cmp::min;
use std::io::Error;
use crate::position::Position;

pub const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION"); 

#[derive(Default, Clone, Copy)]
pub struct Location {
    pub line_index: usize,
    pub grapheme_index: usize,
}

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    show_line_numbers: bool,
    search_info: Option<SearchInfo>,
}

impl UiComponent for View {
    fn mark_redraw(&mut self, val: bool) {
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, position_y: usize) -> Result<(), std::io::Error> {
        let Size { height, width } = self.size;
        let end_y = position_y.saturating_add(height);
        
        #[allow(clippy::integer_division)]
        let top_third = height / 3;
        let scroll_top = self.scroll_offset.row;
        
        let content_width = if self.show_line_numbers {
            width.saturating_sub(6)
        } else {
            width
        };
        
        for current_row in position_y..end_y {
            let line_idx = current_row
                .saturating_sub(position_y)
                .saturating_add(scroll_top);
                
            Terminal::print_row(current_row, &" ".repeat(width))?;
            
            if self.show_line_numbers {
                let line_number = if line_idx < self.buffer.number_of_lines() {
                    format!("{:4}  ", line_idx + 1)
                } else {
                    "     ".to_string()
                };
                
                Terminal::move_caret(Position::new(current_row, 0))?;
                Terminal::print(&line_number)?;
            }
            
            let content_start = if self.show_line_numbers { 6 } else { 0 };
            
            if let Some(line) = self.buffer.lines.get(line_idx) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(content_width);
                let content = line.get_visible_graphemes(left..right);
                
                Terminal::move_caret(Position::new(current_row, content_start))?;
                Terminal::print(&content)?;
            } else if current_row == top_third && self.buffer.is_empty() {
                let message = Self::build_welcome_message(content_width);
                Terminal::move_caret(Position::new(current_row, content_start))?;
                Terminal::print(&message)?;
            } else {
                Terminal::move_caret(Position::new(current_row, content_start))?;
                Terminal::print("~")?;
            }
        }
        Ok(())
    }
}

impl View {

    // === Search === //

    pub fn dimiss_search(&mut self) {
        if let Some(search_info) = &self.search_info {
            self.text_location = search_info.prev_location;
        }
        self.search_info = None;
        self.scroll_text_location_into_view();
    }

    pub fn enter_search(&mut self) {
        self.search_info = Some(
            SearchInfo {
                prev_location: self.text_location
            }
        );
    }

    pub fn exit_search(&mut self) {
        self.search_info = None;
    }

    pub fn search(&mut self, query: &str) {
        if query.is_empty() {
            return;
        }

        if let Some(location) = self.buffer.search(query) {
            self.text_location = location;
            self.scroll_text_location_into_view();
        }
    }

    pub const fn is_file_loaded(&self) -> bool {
        self.buffer.file_info.has_path()
    }

    // === Command Handlers === //

    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(c) => self.insert_character(c),
            Edit::Backspace => self.backspace(),
            Edit::Delete => self.delete(),
            Edit::Enter => self.insert_newline(),
            Edit::RemoveLine => self.delete_line(),
        }
    }

    pub fn handle_move_command(&mut self, command: Move) {
        let Size { height, .. } = self.size;
        match command {
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::PageUp => self.move_up(height.saturating_sub(1)),
            Move::PageDown => self.move_down(height.saturating_sub(1)),
            Move::Home => self.move_to_beggining_of_line(),
            Move::End => self.move_to_end_of_line(),
            Move::WordJumpLeft => self.jump_word_left(),
            Move::WordJumpRight => self.jump_word_right(),
        }
        self.scroll_text_location_into_view();
    }

    // === Saving Files === //

    pub(crate) fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()
    }
    
    pub(crate) fn save_as(&mut self, file_name: &str ) -> Result<(), Error> {
        self.buffer.save_as(file_name)
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        match Buffer::load(file_name) {
            Ok(buffer) => {
                self.buffer = buffer;
                self.mark_redraw(true);
                Ok(())
            }
            Err(error) => Err(error)
        }
    }

    // === Write text === //

    fn insert_character(&mut self, c: char) {
        let old_grapheme_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        self.buffer.insert_char(c, &self.text_location);

        let new_grapheme_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        
        let grapheme_difference = new_grapheme_len.saturating_sub(old_grapheme_len);

        if grapheme_difference > 0 {
            self.handle_move_command(Move::Right);
        }

        self.mark_redraw(true);
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(&self.text_location);
        self.handle_move_command(Move::Right);
        self.mark_redraw(true);
    }

    // === Delete text === //

    fn backspace(&mut self) {
        if self.text_location.grapheme_index == 0 && self.text_location.line_index == 0 {
            return;
        }
        self.handle_move_command(Move::Left);
        self.delete();
    }

    fn delete(&mut self) {
        self.buffer.delete(&self.text_location);
        self.mark_redraw(true);
    }

    fn delete_line(&mut self) {
        self.buffer.delete_line(self.text_location.line_index);
        self.move_up(1);
        self.mark_redraw(true);
    }

    // === Movement functions === //

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_right(&mut self) {
        let line_width = self.buffer.lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_beggining_of_line();
            self.move_down(1);
        }
    }

    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    const fn move_to_beggining_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn jump_word_right(&mut self) {
        if let Some(buffer_line) = self.buffer.lines.get(self.text_location.line_index) {
            let grapheme_count = buffer_line.grapheme_count();

            if self.text_location.grapheme_index >= grapheme_count {
                self.move_to_beggining_of_line();
                self.move_down(1);
                return;
            }

            let mut curr_index = self.text_location.grapheme_index;

            while curr_index < grapheme_count {
                if let Some(fragment) = buffer_line.get_fragments().get(curr_index) {
                    let is_white_space = fragment.grapheme.trim().is_empty() ||
                        fragment.replacement == Some(' ') ||
                        fragment.replacement == Some('␣');
                
                    if is_white_space {
                        break;
                    }

                    curr_index = curr_index.saturating_add(1);
                } else {
                    break;
                }
            }

            while curr_index < grapheme_count {
                if let Some(fragment) = buffer_line.get_fragments().get(curr_index) {
                    let is_white_space = fragment.grapheme.trim().is_empty() ||
                        fragment.replacement == Some(' ') ||
                        fragment.replacement == Some('␣');
                
                    if !is_white_space {
                        break;
                    }

                    curr_index = curr_index.saturating_add(1);
                }else {
                    break;
                }
            }

            if curr_index >= grapheme_count {
                self.move_to_beggining_of_line();
                self.move_down(1);
                return;
            }

            self.text_location.grapheme_index = curr_index;
        }
    }

    fn jump_word_left(&mut self) {
        if self.text_location.line_index == 0 && self.text_location.grapheme_index == 0 {
            return;
        }

        if let Some(buffer_line) = self.buffer.lines.get(self.text_location.line_index) {
            if self.text_location.grapheme_index == 0 {
                self.move_up(1);
                self.move_to_end_of_line();
                return;
            }
    
            let mut curr_index = self.text_location.grapheme_index;
            
            curr_index = curr_index.saturating_sub(1);
            
            while curr_index > 0 {
                if let Some(fragment) = buffer_line.get_fragments().get(curr_index) {
                    let is_white_space = fragment.grapheme.trim().is_empty() ||
                        fragment.replacement == Some(' ') ||
                        fragment.replacement == Some('␣');
                    
                    if !is_white_space {
                        break;
                    }
                    
                    curr_index = curr_index.saturating_sub(1);
                } else {
                    break;
                }
            }
            
            while curr_index > 0 {
                let prev_index = curr_index.saturating_sub(1);
                if let Some(fragment) = buffer_line.get_fragments().get(prev_index) {
                    let is_white_space = fragment.grapheme.trim().is_empty() ||
                        fragment.replacement == Some(' ') ||
                        fragment.replacement == Some('␣');
                    
                    if is_white_space {
                        break;
                    }
                    
                    curr_index = prev_index;
                } else {
                    break;
                }
            }
            
            if curr_index == 0 && self.text_location.line_index > 0 {
                self.move_up(1);
                self.move_to_end_of_line();
                return;
            }
            
            self.text_location.grapheme_index = curr_index;
        }
    }


    // === Scroll === //

    fn scroll_horizontally(&mut self, to: usize) {
        let Size {width, ..} = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.mark_redraw(true);
        }
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size{height, ..} = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.mark_redraw(true);
        }
    }

    // === Fixup functions === //

    fn scroll_text_location_into_view(&mut self) {
        let Position{ row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.number_of_lines());
    }

    // === Other === //

    pub fn toggle_line_numbers(&mut self) {
        let show = self.show_line_numbers;
        self.show_line_numbers = !show;
        self.mark_redraw(true);
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        
        let remainign_width = width.saturating_sub(1);

        if remainign_width < len {
            return "~".to_string();
        }
        
        format!("{:<1}{:^remainign_width$}", "~", welcome_message)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line|{
            line.width_until(self.text_location.grapheme_index)
        });
        
        Position {
            row,
            col
        }
    }

    pub fn get_caret_position(&self) -> Position {
        let mut position = self.text_location_to_position().saturating_sub(self.scroll_offset);
        if self.show_line_numbers {
            position.col = position.col.saturating_add(6);
        }
        position
    }

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            file_name: format!("{}", self.buffer.file_info),
            number_of_lines: self.buffer.number_of_lines(),
            line_number: self.text_location.line_index,
            modified: self.buffer.dirty,
        }
    }
}
