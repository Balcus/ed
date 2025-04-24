#[derive(Default, PartialEq, Eq, Debug)]
pub struct DocumentStatus {
    pub file_name: String,
    pub number_of_lines: usize,
    pub line_number: usize,
    pub modified: bool,
}
impl DocumentStatus {
    pub(crate) fn line_count_to_string(&self) -> String {
        format!("{} lines", self.number_of_lines)
    }
    
    pub(crate) fn modified_indicator_to_string(&self) -> String {
        if self.modified {
            return String::from("(modified)");
        } else {
            String::new()
        }
    }
    
    pub(crate) fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.line_number.saturating_add(1), self.number_of_lines
        )
    }
}