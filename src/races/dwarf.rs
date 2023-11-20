use super::race::{RaceAttributes, Race};

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