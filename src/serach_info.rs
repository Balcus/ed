use crate::line::Line;
use crate::{location::Location, position::Position};

pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Line,
}
