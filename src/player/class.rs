use std::io;

use crate::player::stats::hunter_base_stats;
use crate::player::stats::mage_base_stats;
use crate::player::stats::warrior_base_stats;

use super::stats::BaseStats;

#[derive(Debug)]
pub enum Class {
  Warrior,
  Mage,
  Hunter,
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn class_choose() -> (Class, BaseStats) {
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
    let choose_class = choose_class.trim().to_lowercase();

    // Create Warrior
    if choose_class == "warrior" || choose_class == "1" {
      return (Class::Warrior, warrior_base_stats());
    }
    // Create Mage
    else if choose_class == "mage" || choose_class == "2" {
      return (Class::Mage, mage_base_stats());
    }
    // Create Hunter
    else if choose_class == "hunter" || choose_class == "3" {
      return (Class::Hunter, hunter_base_stats());
    } else {
      println!("Invalid class");
    }
  }
}
