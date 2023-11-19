struct Race {
    name: String,
    dexterity: i32,
}

pub trait RaceTrait {
    fn new() -> ();

    fn create_races_intances() -> i32 {
        unimplemented!()
    }
    fn max_life_points() -> i32;
}

struct Archetype {
    name: String,
    special: String,
    cost: i32,
}

trait ArchetypeTrait {
    fn energy_type() -> EnergyType;
}

enum EnergyType {
    Mana,
    Stamina,
}

struct Energy {
    type_: EnergyType,
    amout: i32
}

impl ArchetypeTrait for Mage {
    fn energy_type() -> EnergyType {
        EnergyType::Mana
    }
}
impl ArchetypeTrait for Necromancer {
    fn energy_type() -> EnergyType {
        EnergyType::Mana
    }
}
impl ArchetypeTrait for Warrior {
    fn energy_type() -> EnergyType {
        EnergyType::Stamina
    }
}
impl ArchetypeTrait for Ranger {
    fn energy_type() -> EnergyType {
        EnergyType::Stamina
    }
}

impl RaceTrait for Dwarf {
    fn max_life_points() -> i32 { 99 }
}

impl RaceTrait for Elf {
    fn new(&self, name: String, dexterity: i32) {
        Race { name, dexterity };
    }

    fn max_life_points() -> i32 { 99 }
}

impl RaceTrait for Halfling {
    fn new(&self, name: String, dexterity: i32) {
        Race { name, dexterity };
    }

    fn max_life_points() -> i32 { 60 }
}

impl RaceTrait for Orc {
    fn new(&self, name: String, dexterity: i32) {
        Race { name, dexterity };
    }

    fn max_life_points() -> i32 { 74 }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_creation() {
        let elf = Race { name: String::from("Elf"), dexterity: 10 };

        assert_eq!(elf.name, String::from("Elf"));
        assert_eq!(elf.dexterity, 10)
    }
}