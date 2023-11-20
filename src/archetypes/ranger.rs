use super::{energy::EnergyType, archetype::{ArchetypeAttributes, Archetype}};


struct Ranger {
    attr: ArchetypeAttributes
}

impl Archetype for Ranger {
    fn new(name: String) -> Self {
        let attr = ArchetypeAttributes { name, special: 0, cost: 0 };            
        Self { attr }
    }

    fn get_energy_type() -> EnergyType {
        EnergyType::Stamina
    }
}