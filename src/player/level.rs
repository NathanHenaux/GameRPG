fn level(){
    // Level up
    let mut level = 1;
    let mut xp = 0;
    let mut xp_max = 100;
    let mut level_up = 100;

    if xp >= xp_max {
        level += 1;
        xp = 0;
        xp_max = level_up;
        level_up += 100;
    }

    // Stats
    hp += 5;
    hp_max += 5;
    strength += 2;

    // Stats Warrior

    if class == "Warrior" {
        blocking += 2;
    }

    // Stats Mage

    else if class == "Mage" {
        mana += 5;
        mana_max += 5;
        intelligence += 2;
    }

    // Stats Hunter

    else if class == "Hunter" {
        power += 5;
        dexterity += 2;
    }

    if level > 1 {
        
    }
}