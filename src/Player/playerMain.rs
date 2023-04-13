use std::io;
use owo_colors::OwoColorize;

fn main(){
    // Player choose class
    let class = io::stdin().read_line(); 

    if class == "Warrior" {
        // Create Warrior
        player = Player::new("Warrior", "Player");
        warriorBaseStats = player.warriorBaseStats();
    } 
    
    else if class == "Mage" {
        // Create Mage
        player = Player::new("Mage", "Player");
        mageBaseStats = player.mageBaseStats();
    } 
    
    else if class == "Hunter" {
        // Create Rogue
        player = Player::new("Hunter", "Player");
        hunterBaseStats = player.hunterBaseStats();
    } else {
        println!("Invalid class");
    }

    // Player choose name
    let name = io::stdin().read_line();

    // Create Player with class
    let player = Player::new(class, name);
    return(player)
}