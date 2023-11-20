use rand::prelude::*;

use crate::app::{
    archetypes::{
        archetype::{Archetype, ArchetypeMethods},
        mage::Mage,
    },
    fighters::fighter::Figther,
    races::{race::{Race, RaceMethods}, elf::Elf},
};

use super::energy::Energy;

struct Character {
    race: Race,
    archetype: Archetype,
    max_life_points: u32,
    life_points: u32,
    strength: u32,
    defense: u32,
    dexterity: u32,
    energy: Energy,
    name: String,
}

impl Character {
    pub fn new(name: String) -> Self {
        let mut rng = thread_rng();
        let min: u8 = 1;
        let max: u8 = 10;
        
        let dexterity = rng.gen_range(min..max) as u32;
        let strength = rng.gen_range(min..max) as u32;
        let defense = rng.gen_range(min..max) as u32;
        let amount = rng.gen_range(min..max) as u32;

        let race = Elf::new(name, dexterity);
        let archetype = Mage::new(name);
        let energy = Energy { type_: Mage::get_energy_type(), amount };
        let max_life_points = race.get_max_life_points() / 2;
        let life_points = race.get_max_life_points();
        

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
    fn receive_damage(&self, attack_point: u32) -> i32 {
        let damage: i32 = self.defense - attack_point;

        if damage > 0 {
            let new_life_point: i32 = self.life_points - damage;

            if new_life_point <= 0 {
                new_life_point = -1;
            }

            self.life_points = new_life_point;
        }

        self.life_points
    }

    fn attack(&mut self, mut enemy: Self) {
        enemy.life_points -= self.strength;
    }

    fn level_up(&self) {
        let mut rng = thread_rng();
        
        let min: u8 = 1;
        let max: u8 = 10;

        let max_life_points_increment = rng.gen_range(min..max);
        let strength_increment = rng.gen_range(min..max);
        let dexterity_points_increment = rng.gen_range(min..max);
        let defense_points_increment = rng.gen_range(min..max);

        self.life_points += max_life_points_increment;

        self.strength += strength_increment;
        self.dexterity += dexterity_points_increment;
        self.defense += defense_points_increment;
        self.energy.amount = 10;
    }

    fn special(enemy: Self) {
        unimplemented!()
    }
}
