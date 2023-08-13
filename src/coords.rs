use std::string;

#[derive(Debug)]
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
    pub fn in_line(&self, line: &Line) -> Option<usize> {
        for (i, coord) in line.coords.iter().enumerate() {
            if self.equals(coord) {
                return Some(i);
            }
        }
        return None
    }
}

#[derive(Debug)]
pub struct Line {
    pub coords: Vec<Coord>
}

impl Line {
    pub fn new() -> Self {
        return Line {coords: Vec::new()}
    }
    pub fn includes_coord(&self, coord: Coord) {

    }
}