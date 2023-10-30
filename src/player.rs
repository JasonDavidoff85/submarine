use crate::{Geometry, Direction, Coord, Sub, BombType, bombs::{Bomb, Sinker}, report::Report};
use core::fmt;
use rand::prelude::*;

const SIZE: usize = 8;

/// [x][z][y]
///    z
///    |
///    |
///    |_______ y North
///   /
///  /
/// x
/// East
type Field = [[[State; SIZE]; SIZE]; SIZE];

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum State {
    Water,
    Sub,
    Wreck,
}

pub struct Player {
    pub field: Field,
    pub balance: i32,
}

impl Player {
    pub fn new() -> Self {
        Player { field: [[[State::Water; SIZE]; SIZE]; SIZE], balance: 1000 }
    }

    pub fn random_coord() -> Coord {
        Coord{
            x: rand::thread_rng().gen_range(0..SIZE),
            y: rand::thread_rng().gen_range(0..SIZE), 
            z: rand::thread_rng().gen_range(0..SIZE)
        }
    }

    fn set_coord(&mut self, coord: &Coord, state: State) {
        self.field[coord.x][coord.z][coord.y] = state;
    }
    
    fn valid_coord(coord: &Coord) -> bool {
        coord.x < SIZE && coord.y < SIZE && coord.z < SIZE
    }

    fn set_geometry(&mut self, geo: &Geometry, state: State) {
        for i in &geo.coords {
            self.set_coord(&i, state);
        }
    }

    fn is_empty(&self, coord: &Coord) -> bool {
        if Self::valid_coord(coord){
            return self.field[coord.x][coord.z][coord.y] == State::Water;
        }
        false
        
    }

    pub fn hits_ship(&self, coord: &Coord) -> bool {
        if Self::valid_coord(coord) {
            return self.field[coord.x][coord.z][coord.y] == State::Sub;
        }
        false
    }

    // pub fn buy_bomb(&mut self, bomb_type: BombType) -> Option<Box<dyn Bomb>> {
    //     match bomb_type {
    //         BombType::Sinker => {
    //             let (val, new_bal) = Sinker::get_bomb(self.balance);
    //             match val {
    //                 Some(x) => {
    //                     self.balance = new_bal;
    //                     return Some(Box::new(x) as Box<Sinker>)
    //                 },
    //                 None => None
    //             }
    //         }
    //         BombType::Strait => None
    //     }
    // }

    pub fn buy_bomb<T: Bomb>(&mut self) -> Result<T, &'static str> {
        let (bomb, new_bal) = T::get_bomb(self.balance);
        match bomb {
            Some(x) => {
                self.balance = new_bal;
                Ok(x)
            },
            None => Err("Insufficient Balance")
        }
    }

    /// Tries to place sub at coord and facing in direction.
    /// Returns true if successful and false if there is something
    /// in the way
    pub fn place_sub(&mut self, sub: &mut Sub, coord: &Coord, dir: Direction) -> bool {
        let mut sline = Geometry::new();
        match dir {
            Direction::North => {
                if coord.y < sub.len - 1 {
                    return false
                }
                for i in 0..sub.len {
                    let c = Coord { x: coord.x, y: coord.y - i, z: coord.z };
                    if self.is_empty(&c) {
                        sline.coords.push(c);
                    } else {
                        return false
                    }
                }
            },
            Direction::South => {
                for i in 0..sub.len {
                    let c = Coord { x: coord.x, y: coord.y + i, z: coord.z };
                    if self.is_empty(&c) {
                        sline.coords.push(c);
                    } else {
                        return false
                    }
                }       
            },
            Direction::East => {
                if coord.x < sub.len - 1 {
                    return false
                }
                for i in 0..sub.len {
                    let c = Coord { x: coord.x - i, y: coord.y, z: coord.z };
                    if self.is_empty(&c) {
                        sline.coords.push(c);
                    } else {
                        return false
                    }
                }       
            },
            Direction::West => {
                if coord.y < sub.len - 1 {
                    return false
                }
                for i in 0..sub.len {
                    let c = Coord { x: coord.x + i, y: coord.y, z: coord.z };
                    if self.is_empty(&c) {
                        sline.coords.push(c);
                    } else {
                        return false
                    }
                    
                }       
            }
        }
        self.set_geometry(&sline, State::Sub);
        sub.geo = sline;
        return true
    }

    pub fn print_face(&self, face: Direction) {
        
        match face {
            Direction::North => {
                // print xz plane
                // x[size] on left
                println!("Z\n");
                for z in (0..SIZE).rev() {
                    print!("{} |", z);
                    for x in (0..SIZE).rev() {
                        let mut total = 0;
                        for y in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{} ", total);
                    }
                    print!("\n  {:-<1$}\n", "", SIZE*2);
                }
                print!("   ");
                for i in (0..SIZE).rev() {
                    print!("{} ", i);
                }
                println!(" X");
            },
            Direction::South => {
                // print xz plane
                // x[0] on leftprint!("   ");
                println!("Z\n");
                for z in (0..SIZE).rev() {
                    print!("{} |", z);
                    for x in 0..SIZE {
                        let mut total = 0;
                        for y in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{} ", total);
                    }
                    print!("\n  {:-<1$}\n", "", SIZE*2);
                }
                print!("   ");
                for i in 0..SIZE {
                    print!("{} ", i);
                }
                println!(" X");
            },
            Direction::East => {
                // print zy plane
                // y[0] on left
                println!("Z\n");
                for z in (0..SIZE).rev() {
                    print!("{} |", z);
                    for y in 0..SIZE {
                        let mut total = 0;
                        for x in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{} ", total);
                    }
                    print!("\n  {:-<1$}\n", "", SIZE*2);
                }
                print!("   ");
                for i in 0..SIZE {
                    print!("{} ", i);
                }
                println!(" Y");
            },
            Direction::West => {
                // print zy plane
                // y[size] on left
                println!("Z\n");
                for z in (0..SIZE).rev() {
                    print!("{} |", z);
                    for y in (0..SIZE).rev() {
                        let mut total = 0;
                        for x in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{} ", total);
                    }
                    print!("\n  {:-<1$}\n", "", SIZE*2);
                }
                print!("   ");
                for i in (0..SIZE).rev() {
                    print!("{} ", i);
                }
                println!(" Y");
            }
        }
    }
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut view = [[0u8; SIZE]; SIZE];
        for (x, i) in self.field.iter().enumerate() {
            for j in 0..i.len() {
                for (y,k) in i.iter().enumerate() {
                    if k[j] == State::Sub {
                        view[x][y] += 1;
                    }
                }
            } 
        }
        for i in view {
            print!("-");
            for j in i {
                print!("{}", j);
            }
            print!("\n");
        }
        write!(f, "\n")
    }
}
