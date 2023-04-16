#![allow(non_snake_case)]
#[allow(unused_variables)]

use Player::PlayerClass::class_choose;

// Modules Player
mod Player;

fn main(){
    // Player Name
    let _name = Player::PlayerName::name_choose();

    // Player Class
    let _class_player = class_choose();

    // Player Main
}