use rand::prelude::*;

use crate::app::{
    archetypes::{
        archetype::{self, ArchetypeAttributes},
        energy::EnergyType,
    },
    fighters::fighter::Figther,
    races::race::RaceAttributes,
};

struct Character {
    race: RaceAttributes,
    archetype: ArchetypeAttributes,
    max_life_points: u32,
    life_points: u32,
    strength: u32,
    defense: u32,
    dexterity: u32,
    energy: EnergyType,
    name: String,
}

impl Character {
    pub fn new(
        race: RaceAttributes,
        archetype: ArchetypeAttributes,
        max_life_points: u32,
        life_points: u32,
        strength: u32,
        defense: u32,
        dexterity: u32,
        energy: EnergyType,
        name: String,
    ) -> Self {
        Self {
            race,
            archetype,
            max_life_points,
            life_points,
            strength,
            defense,
            dexterity,
            energy,
            name,
        }
    }
}

impl Figther for Character {
    fn attack(enemy: Self) {}

    fn level_up() {}

    fn receive_damage(attack_point: u32) -> u32 {

    }

    fn special(enemy: Self) {}
}
