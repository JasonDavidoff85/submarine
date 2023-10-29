mod coords;
mod direction;
mod bombs;
mod player;
mod sub;

use core::fmt;
use std::io;

use direction::{
    Direction,
    random_direction
};
use coords::{
    Coord,
    Line
};
use bombs::BombType;
use player::Player;
use player::State;
use sub::Sub;

use crate::bombs::Bomb;
use crate::bombs::Sinker;


fn main() {

    let mut player = Player::new();
    // let mut sub = Sub::new(3);
    // let mut sub2 = Sub::new(3);
    
    // if !player.place_sub(&mut sub, &Coord { x: 0, y: 2, z: 0}, Direction::North) {
    //     print!("Failed to place sub")
    // }
    // if !player.place_sub(&mut sub2, &Coord { x: 4, y: 3, z: 0}, Direction::West) {
    //     print!("Failed to place sub")
    // }

    
    const NUM_SHIPS: usize = 5;
    let ship_sizes: [usize; NUM_SHIPS] = [2, 3, 4, 4, 5];
    for i in ship_sizes {
        loop {
            let mut s = Sub::new(i);
            if !player.place_sub(&mut s, &Player::random_coord(), random_direction()) {
                continue;
            } else {
                println!("Added: {:#?}", s);
                break
            }
        }
    }

    println!("North");
    player.print_face(Direction::North);
    println!("\nSouth");
    player.print_face(Direction::South);
    println!("\nEast");
    player.print_face(Direction::East);
    println!("\nWest");
    player.print_face(Direction::West);
    

    // loop {
    //     let mut guess = String::new();
    //     println!(">> ");
    //     io::stdin().read_line(&mut guess).expect("failed to readline");
    //     println!("{}", guess)
    // }
    
    // get bomb example
    // let mut bomb = player.buy_bomb::<Sinker>().unwrap();
    // bomb.coord = Some(Coord{x: 4, y: 4, z: 4});
    // bomb.radius = 3;
    // let geo = bomb.get_geometry().unwrap();
    // print!{"{:?}\n", geo}
    // for i in geo.coords {
    //     player.field[i.x][i.z][i.y] = State::Sub
    // }

    // player.print_face(Direction::North);
    // bomb.dir = Direction::North;
    // player.launch_bomb(player2, bomb);
}
