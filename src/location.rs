use crate::terminal::Position;

#[derive(Copy, Clone, Default, Debug)]
pub struct  Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(location: Location) -> Self {
        Self {
            col: location.x,
            row: location.y,
        }
    }
}

impl Location {
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}