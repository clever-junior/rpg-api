use super::energy::EnergyType;

pub struct ArchetypeAttributes {
    pub name: String,
    pub special: u32,
    pub cost: u32,
}

pub trait Archetype {
    fn new(name: String) -> Self;

    fn get_energy_type() -> EnergyType;
}