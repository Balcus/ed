use crate::line::Line;
use crate::{position::Position, view::location::Location};

pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Option<Line>,
}
