#![allow(non_snake_case)]
#[allow(unused_variables)]

use player::class::class_choose;

// Modules Player
mod player;

fn main(){
    // Player Name
    let name = player::name::name_choose();

    // Player Class
    let class_player = class_choose();
    println!("You have chosen the class: {}", class_name)
}