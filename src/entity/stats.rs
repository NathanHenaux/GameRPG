pub struct base_stats_entity {
    hp: u16,
    hp_max: u16,
    strength: u16,
}

// Stats of entity change every level
pub fn wolf() {
    let wolf_stats = base_stats_entity {
        hp: 100,
        hp_max: 100,
        strength: 10,
    };
}

pub fn thief(){
    let thief_stats = base_stats_entity {
        hp: 150,
        hp_max: 150,
        strength: 15,
    };
}

pub fn chief_thief(){
    let chief_thief_stats = base_stats_entity {
        hp: 250,
        hp_max: 250,
        strength: 20,
    };
}