mod coords;
use core::fmt;

use coords::Coord;
use coords::Line;

mod sub;
use sub::Sub;

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

enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Clone, Copy)]
#[derive(PartialEq)]
enum State {
    Water,
    Sub,
    Wreck,
}

struct Player {
    field: Field,
}

impl Player {
    fn new() -> Self {
        Player { field: [[[State::Water; SIZE]; SIZE]; SIZE] }
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




fn main() {

    // let mut player = Player::new();
    // let a = Coord {x: 1, y: 2, z: 3};
    // let mut line = Line::new();
    // line.coords.push(Coord {x: 1,y: 2,z: 3});
    // let b = Coord {x: 1, y: 2, z: 3};

    // let result = a.in_line(&line);
    // match result {
    //     Some(x) => println!("It is in the line at index {}", x),
    //     None => println!("Not found")

    // }

    let mut player = Player::new();
    let mut sub = Sub::new(3);
    let mut sub2 = Sub::new(3);
    
    println!("{}",player.place_sub(&mut sub, &Coord { x: 0, y: 2, z: 0}, Direction::North));
    println!("{}",player.place_sub(&mut sub2, &Coord { x: 4, y: 3, z: 0}, Direction::West));
    
    player.print_face(Direction::North);
}

// fn print_field(field: &Field) {
//     println!("   0   1   2   3   4   5   6   7");
//     for (i, x) in field.iter().enumerate() {
//         print!("{}",i);
//         for y in x {
//             if *y {
//                 print!("| x ")
//             } else {
//                 print!("|   ")
//             }
//         }
//         print!("|\n  --------------------------------\n");
//     }
// }
