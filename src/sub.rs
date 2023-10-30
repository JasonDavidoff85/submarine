use crate::{Geometry, Direction, Coord};

#[derive(Debug)]
pub struct Sub {
    pub id: u8,
    pub len: usize,
    pub geo: Geometry,
}

impl Sub {
    pub fn new(len: usize) -> Self {
        Sub {
            len,
            id: 0,
            geo: Geometry::new()
        }
    }
}