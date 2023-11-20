use crate::app::energy::EnergyType;

use super::{
    archetype::{Archetype, ArchetypeMethods},
};

struct Necromancer {
    arquetype: Archetype,
}

impl ArchetypeMethods for Necromancer {
    fn new(name: String) -> Self {
        let arquetype = Archetype {
            name,
            special: 0,
            cost: 0,
        };
        Self { arquetype }
    }

    fn get_cost(&self) -> u32 {
        self.arquetype.cost
    }

    fn get_name(&self) -> String {
        self.arquetype.name
    }

    fn get_special(&self) -> u32 {
        self.arquetype.special
    }

    fn get_energy_type() -> EnergyType {
        EnergyType::Mana
    }
}
