#![allow(non_snake_case)]
#[allow(unused_variables)]

use player::class::class_choose;

// Modules Player
mod player;

fn main(){
    // Player Name
    let _name = player::name::name_choose();

    // Player Class
    let _class_player = class_choose();

    // Player Main
}