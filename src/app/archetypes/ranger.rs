use crate::app::energy::EnergyType;

use super::archetype::{Archetype, ArchetypeMethods};

struct Ranger {
    archetype: Archetype,
}

impl ArchetypeMethods for Ranger {
    fn new(name: String) -> Self {
        let archetype = Archetype {
            name,
            special: 0,
            cost: 0,
        };
        Self { archetype }
    }

    fn get_cost(&self) -> u32 {
        self.archetype.cost
    }

    fn get_name(&self) -> String {
        self.archetype.name
    }

    fn get_special(&self) -> u32 {
        self.archetype.special
    }

    fn get_energy_type() -> EnergyType {
        EnergyType::Stamina
    }
}
