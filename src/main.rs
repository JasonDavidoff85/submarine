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
type Field = [[[bool; SIZE]; SIZE]; SIZE];

enum Direction {
    North,
    South,
    East,
    West
}
struct Player {
    field: Field,
}

impl Player {
    fn new() -> Self {
        Player { field: [[[false; SIZE]; SIZE]; SIZE] }
    }

    fn set_coord(&mut self, coord: &Coord) {
        self.field[coord.x][coord.z][coord.y] = true;
    }

    fn set_line(&mut self, line: &Line) {
        for i in &line.coords {
            self.set_coord(&i);
        }
    }

    fn is_empty(&self, coord: &Coord) -> bool {
        if coord.x < SIZE && coord.y < SIZE && coord.z < SIZE {
            self.field[coord.x][coord.y][coord.z] == false
        } else {
            return false
        }
        
    }

    pub fn place_sub(&mut self, sub: &mut Sub, coord: &Coord, dir: Direction) -> bool {
        let mut sline = Line::new();
        match dir {
            Direction::North => {
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
                for i in 0..sub.len {
                    let c = Coord { x: coord.x + i, y: coord.y - i, z: coord.z };
                    if self.is_empty(&c) {
                        sline.coords.push(c);
                    } else {
                        return false
                    }
                    
                }       
            }
        }
        self.set_line(&sline);
        sub.line = sline;
        return true
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut view = [[0u8; SIZE]; SIZE];
        for (x, i) in self.field.iter().enumerate() {
            for j in 0..i.len() {
                for (y,k) in i.iter().enumerate() {
                    if k[j] {
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
    let a = Coord {x: 1, y: 2, z: 3};
    let mut line = Line::new();
    line.coords.push(Coord {x: 1,y: 2,z: 3});
    let b = Coord {x: 1, y: 2, z: 3};

    // let result = a.in_line(&line);
    // match result {
    //     Some(x) => println!("It is in the line at index {}", x),
    //     None => println!("Not found")

    // }

    let mut player = Player::new();
    let mut sub = Sub::new(3);
    println!("{}",player.place_sub(&mut sub, &Coord { x: 0, y: 0, z: 0 }, Direction::North));
    println!("{}", player);
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
