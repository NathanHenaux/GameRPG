use super::PlayerName;
use super::PlayerClass;

#[allow(dead_code)]
pub fn Player() {
    // Player Name
    PlayerName::name_choose();

    // Player Class 
    let class = PlayerClass::class_choose();
    println!("Your class is {:?}", class);
}