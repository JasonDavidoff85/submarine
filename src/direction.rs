use rand::prelude::*;

pub enum Direction {
    North,
    South,
    East,
    West
}

pub fn random_direction() -> Direction {
    match rand::thread_rng().gen_range(0..4) {
        0 => Direction::North,
        1 => Direction::South,
        2 => Direction::East,
        3 => Direction::West,
        _ => Direction::North,
    }
}