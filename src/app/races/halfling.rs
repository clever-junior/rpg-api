use super::race::{Race, RaceMethods};

struct Halfling {
    attr: Race,
    max_life_points: u32,
}

impl RaceMethods for Halfling {
    fn new(name: String, dexterity: u32) -> Self {
        let attr = Race { name, dexterity };

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