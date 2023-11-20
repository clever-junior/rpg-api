use super::{energy::EnergyType, archetype::{ArchetypeAttributes, Archetype}};


struct Necromancer {
    attr: ArchetypeAttributes
}

impl Archetype for Necromancer {
    fn new(name: String) -> Self {
        let attr = ArchetypeAttributes { name, special: 0, cost: 0 };            
        Self { attr }
    }

    fn get_energy_type() -> EnergyType {
        EnergyType::Mana
    }
}