use super::{
    archetype::{Archetype, ArchetypeAttributes},
    energy::EnergyType,
};

struct Mage {
    attr: ArchetypeAttributes,
}

impl Archetype for Mage {
    fn new(name: String) -> Self {
        let attr = ArchetypeAttributes {
            name,
            special: 0,
            cost: 0,
        };
        Self { attr }
    }

    fn get_energy_type() -> EnergyType {
        EnergyType::Mana
    }
}
