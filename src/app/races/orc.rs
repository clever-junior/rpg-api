use super::race::{Race, RaceMethods};

struct Orc {
    attr: Race,
    max_life_points: u32
}

impl RaceMethods for Orc {
    fn new(name: String, dexterity: u32) -> Self {
        let attr = Race { name, dexterity };

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