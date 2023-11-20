use super::race::{RaceAttributes, Race};

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