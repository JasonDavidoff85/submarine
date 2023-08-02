use crate::{Line, Direction, Coord};

pub struct Sub {
    pub id: u8,
    pub len: usize,
    pub line: Line,
}

impl Sub {
    pub fn new(len: usize) -> Self {
        Sub {
            len,
            id: 0,
            line: Line::new()
        }
    }
}