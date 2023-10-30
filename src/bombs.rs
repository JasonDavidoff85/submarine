// types of bombs:
// stright line goes across field
// drop bombs explode with radius

use crate::{coords::{Geometry, Coord}, direction::Direction, report::Report, player::Player};
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
    fn get_geometry(&self) -> Option<Geometry>;
    fn get_bomb(balance: i32 ) -> (Option<Self> , i32) where Self: Sized;
    fn launch(&self, player: &mut Player) -> Report;
}

impl Bomb for Sinker {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bomb_type(&self) -> BombType {
        BombType::Sinker
    }

    fn get_geometry(&self) -> Option<Geometry> {
        match &self.coord {
            Some(coord) => {
                let mut area = Geometry::new();
                for i in 0..self.radius {
                    // area.coords.push(Coord{x: coord.x + i as usize, y: coord.y, z: coord.z });
                    // area.coords.push(Coord{x: coord.x, y: coord.y + i as usize, z: coord.z });
                    // area.coords.push(Coord{x: coord.x, y: coord.y, z: coord.z + i as usize});
                    // area.coords.push(Coord{x: coord.x - i as usize, y: coord.y, z: coord.z });
                    // area.coords.push(Coord{x: coord.x, y: coord.y - i as usize, z: coord.z });
                    // area.coords.push(Coord{x: coord.x, y: coord.y, z: coord.z - i as usize});
                    area.coords.push(Coord{x: coord.x as usize, y: coord.y, z: coord.z });
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

    fn launch(&self, player: &mut Player) -> Report {
        let mut report = Report::new();
        for i in self.get_geometry().unwrap().coords {
            report.area.coords.push(i);
            if player.hits_ship(&i) {
                report.hit.coords.push(i);
            }
        }
        report
    }
}


