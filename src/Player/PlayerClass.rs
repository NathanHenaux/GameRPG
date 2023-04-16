use std::io;

use crate::Player::PlayerStats::warrior_base_stats;
use crate::Player::PlayerStats::mage_base_stats;
use crate::Player::PlayerStats::hunter_base_stats;

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn class_choose() {

    loop {	
        // Player choose class
        println!("Choose your class: ");
        println!("1. Warrior");
        println!("2. Mage");
        println!("3. Hunter");

        let mut choose_class = String::new();
        io::stdin().read_line(&mut choose_class).expect("Failed to read line");
        let choose_class = choose_class.trim();

        // Lookup choose class

        let class_warrior = choose_class == "Warrior" || choose_class == "warrior" || choose_class == "1";
        let class_mage = choose_class == "Mage" || choose_class == "mage" || choose_class == "2";
        let class_hunter = choose_class == "Hunter" || choose_class == "hunter" || choose_class == "3";

        // Create Warrior

        if class_warrior {
            let class = "Warrior";
            let mut stats = warrior_base_stats();
            break;
        }
        
        // Create Mage

        else if class_mage {
            let class = "Mage";
            let mut stats = mage_base_stats();
            break;
        } 
        
        // Create Hunter

        else if class_hunter {
            let class = "Hunter";
            let mut stats = hunter_base_stats();
            break;
        }
        
        else {
            println!("Invalid class");
        }
    }
}