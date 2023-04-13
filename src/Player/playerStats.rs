// Type de stats

// Point de vie = hp
// Point de vie Max = hpMax
// Mana = mana
// Mana Max = manaMax
// Rage = rage
// Rage Max = rageMax
// Energie = power
// Energie Max = powerMax
// Armure = armor
// Force = strength 
// Agilité = agility
// Intelligence = intelligence
// Dextérité = dexterity
// Esquive = dodging
// Blocage = blocking

pub struct baseStats {
    hp: u16,
    hp_max: u16,
    strength: u16,
    armor: u16,

    mana: u16,      // Mage
    mana_max: u16,
    intelligence: u16,
    dodging: u8,

    rage: u8,      // Warrior
    rage_max: u8,
    blocking: u8,

    power: u16,     // Hunter
    power_max: u16,
    dexterity: u16,
    dodging: u8,
}

pub fn warriorBaseStats() {
    let warriorBaseStats = baseStats {
        hp: 100,
        hp_max: 100,
        strength: 10,
        armor: 10,
        rage: 0,
        rage_max: 100,
        blocking: 0,
    };
}

pub fn mageBaseStats() {
    let mageBaseStats = baseStats {
        hp: 100,
        hp_max: 100,
        strength: 10,
        armor: 10,
        mana: 100,
        mana_max: 100,
        intelligence: 10,
        dodging: 0,
    };
}

pub fn hunterBaseStats() {
    let hunterBaseStats = baseStats {
        hp: 100,
        hp_max: 100,
        strength: 10,
        armor: 10,
        power: 100,
        power_max: 100,
        dodging: 0,
        dexterity: 10,
    };
}