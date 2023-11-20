use race::Race;

pub mod energy {
    #[derive(PartialEq, Debug)]
    pub enum EnergyType {
        Mana,
        Stamina
    }
    
    pub struct Energy {
        type_: EnergyType,
        amount: u32
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_energy() {
            let energy = Energy { type_: EnergyType::Stamina, amount: 10 };
            let energy2 = Energy { type_: EnergyType::Mana, amount: 45 };

            assert_eq!(energy.amount, 10);
            assert_eq!(energy.type_, EnergyType::Stamina);

            assert_eq!(energy2.amount, 45);
            assert_eq!(energy2.type_, EnergyType::Mana);
        }
    }
}

pub mod archetype {
    use crate::energy::EnergyType;

    pub struct ArchetypeAttributes {
        pub name: String,
        pub special: u32,
        pub cost: u32,
    }

    pub trait Archetype {
        fn new(name: String) -> Self;

        fn get_energy_type() -> EnergyType;
    }
}

mod mage {
    use crate::{archetype::{ArchetypeAttributes, Archetype}, energy::EnergyType};

    struct Mage {
        attr: ArchetypeAttributes
    }

    impl Archetype for Mage {
        fn new(name: String) -> Self {
            let attr = ArchetypeAttributes { name, special: 0, cost: 0 };            
            Self { attr }
        }

        fn get_energy_type() -> crate::energy::EnergyType {
            EnergyType::Mana
        }
    }
}

mod necromancer {
    use crate::{archetype::{ArchetypeAttributes, Archetype}, energy::EnergyType};

    struct Necromancer {
        attr: ArchetypeAttributes
    }

    impl Archetype for Necromancer {
        fn new(name: String) -> Self {
            let attr = ArchetypeAttributes { name, special: 0, cost: 0 };            
            Self { attr }
        }

        fn get_energy_type() -> crate::energy::EnergyType {
            EnergyType::Mana
        }
    }
}

mod warrior {
    use crate::{archetype::{ArchetypeAttributes, Archetype}, energy::EnergyType};

    struct Warrior {
        attr: ArchetypeAttributes
    }

    impl Archetype for Warrior {
        fn new(name: String) -> Self {
            let attr = ArchetypeAttributes { name, special: 0, cost: 0 };            
            Self { attr }
        }

        fn get_energy_type() -> crate::energy::EnergyType {
            EnergyType::Stamina
        }
    }
}

mod ranger {
    use crate::{archetype::{ArchetypeAttributes, Archetype}, energy::EnergyType};

    struct Ranger {
        attr: ArchetypeAttributes
    }

    impl Archetype for Ranger {
        fn new(name: String) -> Self {
            let attr = ArchetypeAttributes { name, special: 0, cost: 0 };            
            Self { attr }
        }

        fn get_energy_type() -> crate::energy::EnergyType {
            EnergyType::Stamina
        }
    }
}

pub mod race {
    pub struct RaceAttributes {
        pub name: String,
        pub dexterity: u32,
    }

    pub trait Race {
        fn new(name: String, dexterity: u32) -> Self;

        fn get_name(&self) -> &str;

        fn get_dexterity(&self) -> u32;

        fn get_max_life_points(&self) -> u32;

        fn created_races_intances(&self) -> u32 {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct SomeRace {
            attr: RaceAttributes,
            max_life_points: u32,
        }

        impl Race for SomeRace {
            fn new(name: String, dexterity: u32) -> Self {
                Self {
                    attr: RaceAttributes { name, dexterity },
                    max_life_points: 10,
                }
            }

            fn get_name(&self) -> &str {
                &self.attr.name
            }

            fn get_dexterity(&self) -> u32 {
                self.attr.dexterity
            }

            fn get_max_life_points(&self) -> u32 {
                self.max_life_points
            }
        }

        #[test]
        fn test_race_attr() {
            let race = RaceAttributes {
                name: String::from("Name"),
                dexterity: 0,
            };

            assert!(race.name.len() > 0);
            assert_eq!(race.name, String::from("Name"));
            assert_eq!(race.dexterity, 0);
        }

        #[test]
        fn test_race_impl() {
            let some_race = SomeRace::new(String::from("Name"), 0);

            assert_eq!(some_race.get_name(), String::from("Name"));
            assert_eq!(some_race.get_dexterity(), 0);
            assert_eq!(some_race.get_max_life_points(), 10);
        }
    }
}

mod dwarf {
    use crate::race::{Race, RaceAttributes};

    pub struct Dwarf {
        attr: RaceAttributes,
        max_life_points: u32
    }

    impl Race for Dwarf {
        fn new(name: String, dexterity: u32) -> Self {
            let attr = RaceAttributes { name, dexterity };

            Self { attr, max_life_points: 86 }
        }

        fn get_name(&self) -> &str { &self.attr.name }

        fn get_dexterity(&self) -> u32 {
            self.attr.dexterity
        }

        fn get_max_life_points(&self) -> u32 {
            self.max_life_points
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_race_creation() {
            let dwarf = Dwarf::new(String::from("Dwarf"), 10);

            assert_eq!(dwarf.get_name(), String::from("Dwarf"));
            assert_eq!(dwarf.get_dexterity(), 10);
            assert_eq!(dwarf.get_max_life_points(), 86);
        }
    }
}

mod elf {
    use crate::race::{ Race, RaceAttributes };

    struct Elf {
        attr: RaceAttributes,
        max_life_points: u32
    }

    impl Race for Elf {
        fn new(name: String, dexterity: u32) -> Self {
            let attr = RaceAttributes { name, dexterity };

            Self { attr, max_life_points: 99 }
        }

        fn get_name(&self) -> &str {
            &self.attr.name
        }

        fn get_dexterity(&self) -> u32 {
            self.attr.dexterity
        }
        
        fn get_max_life_points(&self) -> u32 {
            self.max_life_points
        }
    }

    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_race_creation() {
            let elf = Elf::new(String::from("Legolas"), 10);

            assert_eq!(elf.get_max_life_points(), 99)
        }
    }
}

mod halfling {
    use crate::race::{RaceAttributes, Race};

    struct Halfling {
        attr: RaceAttributes,
        max_life_points: u32,
    }

    impl Race for Halfling {
        fn new(name: String, dexterity: u32) -> Self {
            let attr = RaceAttributes { name, dexterity };

            Self { attr, max_life_points: 60 }
        }

        fn get_dexterity(&self) -> u32 {
            self.attr.dexterity
        }

        fn get_max_life_points(&self) -> u32 {
            self.max_life_points   
        }

        fn get_name(&self) -> &str {
            &self.attr.name
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_race_creation() {
            let halfling = Halfling::new(String::from("Name"), 10);

            assert_eq!(halfling.max_life_points, 60);
        }
    }
}

mod orc {
    use crate::race::{RaceAttributes, Race};

    struct Orc {
        attr: RaceAttributes,
        max_life_points: u32
    }

    impl Race for Orc {
        fn new(name: String, dexterity: u32) -> Self {
            let attr = RaceAttributes { name, dexterity };

            Self { attr, max_life_points: 74 }
        }

        fn get_name(&self) -> &str {
            &self.attr.name
        }

        fn get_dexterity(&self) -> u32 {
            self.attr.dexterity
        }

        fn get_max_life_points(&self) -> u32 {
            self.max_life_points
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_race_creation() {
            let orc = Orc::new(String::from("name"), 10);

            assert_eq!(orc.get_max_life_points(), 74);
        }
    }
}

fn main() {

}
