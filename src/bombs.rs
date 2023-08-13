// types of bombs:
// stright line goes across field
// drop bombs explode with radius

use crate::{coords::{Line, Coord}, direction::Direction};
use std::marker::Sized;
use std::any::Any;

#[derive(PartialEq)]
pub enum BombType {
    Sinker,
    Strait
}

pub struct Sinker {
    pub coord: Option<Coord>,
    pub radius: u8
}

pub struct Straight {
    coord: Option<Coord>,
    direction: Option<Direction>
}


pub trait Bomb {
    fn as_any(&self) -> &dyn Any;
    fn bomb_type(&self) -> BombType;
    fn get_geometry(&self) -> Option<Line>;
    fn get_bomb(balance: i32 ) -> (Option<Self> , i32) where Self: Sized;
}

impl Bomb for Sinker {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bomb_type(&self) -> BombType {
        BombType::Sinker
    }

    fn get_geometry(&self) -> Option<Line> {
        match &self.coord {
            Some(coord) => {
                let mut area = Line::new();
                for i in 1..self.radius {
                    area.coords.push(Coord{x: coord.x + i as usize, y: coord.y, z: coord.z });
                    area.coords.push(Coord{x: coord.x, y: coord.y + i as usize, z: coord.z });
                    area.coords.push(Coord{x: coord.x, y: coord.y, z: coord.z + i as usize});
                    area.coords.push(Coord{x: coord.x - i as usize, y: coord.y, z: coord.z });
                    area.coords.push(Coord{x: coord.x, y: coord.y - i as usize, z: coord.z });
                    area.coords.push(Coord{x: coord.x, y: coord.y, z: coord.z - i as usize});
                }
                Some(area)
            },
            None => None
        }
    }
    fn get_bomb(balance: i32) -> (Option<Self> , i32) where Self: Sized {
        if balance >= 5 {
            (Some(Sinker {coord: None, radius: 1}), balance - 5)
        } else {
            (None, balance)
        }
    }
}


