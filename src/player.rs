use crate::{Line, Direction, Coord, Sub, BombType, bombs::{Bomb, Sinker}};
use core::fmt;

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

    fn set_coord(&mut self, coord: &Coord, state: State) {
        self.field[coord.x][coord.z][coord.y] = state;
    }

    fn set_line(&mut self, line: &Line, state: State) {
        for i in &line.coords {
            self.set_coord(&i, state);
        }
    }

    fn is_empty(&self, coord: &Coord) -> bool {
        if coord.x < SIZE && coord.y < SIZE && coord.z < SIZE {
            self.field[coord.x][coord.z][coord.y] == State::Water
        } else {
            return false
        }
        
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

    pub fn buy_bomb<T: Bomb>(&mut self) -> Option<T> {
        let (bomb, new_bal) = T::get_bomb(self.balance);
        match bomb {
            Some(x) => {
                self.balance = new_bal;
                Some(x)
            },
            None => None
        }
    }

    pub fn place_sub(&mut self, sub: &mut Sub, coord: &Coord, dir: Direction) -> bool {
        let mut sline = Line::new();
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
        self.set_line(&sline, State::Sub);
        sub.line = sline;
        return true
    }

    pub fn print_face(&self, face: Direction) {
        match face {
            Direction::North => {
                // print xz plane
                // x[size] on left
                for z in (0..SIZE).rev() {
                    for x in (0..SIZE).rev() {
                        let mut total = 0;
                        for y in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{}", total);
                    }
                    print!("\n")
                }
            },
            Direction::South => {
                // print xz plane
                // x[0] on left
                for z in (0..SIZE).rev() {
                    for x in 0..SIZE {
                        let mut total = 0;
                        for y in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{}", total);
                    }
                    print!("\n")
                }
            },
            Direction::East => {
                // print zy plane
                // y[0] on left
                for z in (0..SIZE).rev() {
                    for y in 0..SIZE {
                        let mut total = 0;
                        for x in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{}", total);
                    }
                    print!("\n")
                }
            },
            Direction::West => {
                // print zy plane
                // y[size] on left
                for z in (0..SIZE).rev() {
                    for y in (0..SIZE).rev() {
                        let mut total = 0;
                        for x in 0..SIZE {
                            if self.field[x][z][y] == State::Sub {
                                total += 1;
                            }
                        }
                        print!("{}", total);
                    }
                    print!("\n")
                }
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
            for j in i {
                print!("{}", j);
            }
            print!("\n");
        }
        write!(f, "\n")
    }
}
