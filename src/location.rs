#[derive(Default, Clone, Copy)]
pub struct Location {
    pub line_index: usize,
    pub grapheme_index: usize,
}
