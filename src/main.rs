#![allow(non_snake_case)]
#[allow(unused_variables)]
use player::class::class_choose;

// Modules Player
mod player;

fn main() {
  // Player Name
  let _name = player::name::name_choose();

  let mut console_reader = std::io::stdin().lock();

  // Player Class
  let (class, _stats) = class_choose(&mut console_reader);
  println!("You have chosen the class: {class:?}")
}
