use std::string;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl Coord {
    pub fn equals(&self, other: &Coord) -> bool {
        match other {
            Coord {x, y, z} => return x == &self.x && y == &self.y && z == &self.z
        }
    }

    /// returns index of where in line
    pub fn in_line(&self, line: &Geometry) -> Option<usize> {
        for (i, coord) in line.coords.iter().enumerate() {
            if self.equals(coord) {
                return Some(i);
            }
        }
        return None
    }
}

#[derive(Debug)]
pub struct Geometry {
    pub coords: Vec<Coord>
}

impl Geometry {
    pub fn new() -> Self {
        return Geometry {coords: Vec::new()}
    }
    pub fn includes_coord(&self, coord: Coord) {

    }
}
