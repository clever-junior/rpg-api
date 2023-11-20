use super::race::{Race, RaceMethods};

pub struct Elf {
    race: Race,
    max_life_points: u32
}

impl RaceMethods for Elf {
    fn new(name: String, dexterity: u32) -> Self {
        let race = Race { name, dexterity };

        Self { race, max_life_points: 99 }
    }

    fn get_name(&self) -> &str {
        &self.race.name
    }

    fn get_dexterity(&self) -> u32 {
        self.race.dexterity
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