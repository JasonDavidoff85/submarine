mod coords;
mod direction;
mod bombs;

use core::fmt;

use direction::Direction;
use coords::Coord;
use coords::Line;
use bombs::BombType;

mod player;
use player::Player;
use player::State;
mod sub;
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
    
    
    
    let mut bomb = player.buy_bomb::<Sinker>().unwrap();
    bomb.coord = Some(Coord{x: 4, y: 4, z: 4});
    let geo = bomb.get_geometry().unwrap();
    print!{"{:?}\n", geo}
    for i in geo.coords {
        player.field[i.x][i.z][i.y] = State::Sub
    }

    player.print_face(Direction::North);
    // bomb.dir = Direction::North;
    // player.launch_bomb(player2, bomb);
}
