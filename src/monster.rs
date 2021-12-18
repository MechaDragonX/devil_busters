use std::fmt;

use crate::resistance::Resistance;

pub struct Monster {
    // Basic Info
    pub name: String,
    pub race: String,
    pub level: i32,
    // HP and MP in Battle
    pub health: i32,
    pub mana: i32,
    // Stat Distrubution
    // Order: Strength, Vitality, Magic, Agility, Luck
    pub stats: [i32; 5],
    // Element Resistances
    // Order: Fire, Ice, Lightning, Wind, Light, Dark
    pub element_res: [Resistance; 7],
    // Ailment Resistances
    // Order: Freeze, Shock, Poison, Paralysis, Stun, Confusion, Charm, Stone, Sleep
    pub ailment_res: [Resistance; 9]
}

impl Monster {
    pub fn new(name: &str, race: &str, level: i32, health: i32, mana: i32) -> Self {
        Monster {
            name: name.to_string(),
            race: race.to_string(),
            level,
            health,
            mana,
            stats: [0, 0, 0, 0, 0],
            element_res: [ Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal ],
            ailment_res: [ Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal ]
        }
    }

    pub fn dist_stats(&mut self, strength: i32, vitality: i32, magic: i32, agility: i32, luck: i32) -> () {
        self.stats[0] = strength;
        self.stats[1] = vitality;
        self.stats[2] = magic;
        self.stats[3] = agility;
        self.stats[4] = luck;
    }
    pub fn dist_element_res(&mut self, physical: Resistance, fire: Resistance, ice: Resistance, lightning: Resistance, wind: Resistance, light: Resistance, dark: Resistance) -> () {
        self.element_res[0] = physical;
        self.element_res[1] = fire;
        self.element_res[2] = ice;
        self.element_res[3] = lightning;
        self.element_res[4] = wind;
        self.element_res[5] = light;
        self.element_res[6] = dark;
    }
    pub fn dist_ailment_res(&mut self, freeze: Resistance, shock: Resistance, poison: Resistance, paralysis: Resistance, stun: Resistance, confusion: Resistance, charm: Resistance, stone: Resistance, sleep: Resistance) -> () {
        self.ailment_res[0] = freeze;
        self.ailment_res[1] = shock;
        self.ailment_res[2] = poison;
        self.ailment_res[3] = paralysis;
        self.ailment_res[4] = stun;
        self.ailment_res[5] = confusion;
        self.ailment_res[6] = charm;
        self.ailment_res[7] = stone;
        self.ailment_res[8] = sleep;
    }
}

impl fmt::Display for Monster {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = format!("{} {1}\nLevel: {2}\nHP: {3}\nMP: {4}", self.race, self.name, self.level, self.health, self.mana);
        result += format!("\n\nStrength: {0}\nVitality: {1}\nMagic: {2}\nAgility: {3}\nLuck: {4}",
            self.stats[0],
            self.stats[1],
            self.stats[2],
            self.stats[3],
            self.stats[4]
        ).as_str();
        result += format!("\n\nElement Resistances:\nPhysical: {0}\nFire: {1}\nIce: {2}\nLightning: {3}\nWind: {4}\nLight: {5}\nDark: {6}",
            self.element_res[0],
            self.element_res[1],
            self.element_res[2],
            self.element_res[3],
            self.element_res[4],
            self.element_res[5],
            self.element_res[6]
        ).as_str();
        result += format!("\n\nAilment Resistances:\nFreeze: {0}\nShock: {1}\nPoison: {2}\nParalysis: {3}\nStun: {4}\nConfusion: {5}\nCharm: {6}\nStone: {7}\nSleep: {8}",
            self.ailment_res[0],
            self.ailment_res[1],
            self.ailment_res[2],
            self.ailment_res[3],
            self.ailment_res[4],
            self.ailment_res[5],
            self.ailment_res[6],
            self.ailment_res[7],
            self.ailment_res[8],
        ).as_str();

        return write!(fmt, "{}", result);
    }
}
