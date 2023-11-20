use crate::app::energy::EnergyType;

pub struct Archetype {
    pub name: String,
    pub special: u32,
    pub cost: u32,
}

pub trait ArchetypeMethods {
    fn new(name: String) -> Self;

    fn get_name(&self) -> String;

    fn get_special(&self) -> u32;

    fn get_cost(&self) -> u32;

    fn get_energy_type() -> EnergyType;
}
