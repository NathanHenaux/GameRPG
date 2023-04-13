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
    hpMax: u16,
    strength: u16,
    armor: u16,

    mana: u16,      // Mage
    manaMax: u16,
    intelligence: u16,
    dodging: u8,

    rage: u16,      // Warrior
    rageMax: u16,
    blocking: u8,

    power: u16,     // Hunter
    powerMax: u16,
    dodging: u8,
    dexterity: u16,
}
