#[derive(PartialEq, Debug)]
pub enum EnergyType {
    Mana,
    Stamina
}

pub struct Energy {
    pub type_: EnergyType,
    pub amount: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let energy = Energy { type_: EnergyType::Stamina, amount: 10 };
        let energy2 = Energy { type_: EnergyType::Mana, amount: 45 };

        assert_eq!(energy.amount, 10);
        assert_eq!(energy.type_, EnergyType::Stamina);

        assert_eq!(energy2.amount, 45);
        assert_eq!(energy2.type_, EnergyType::Mana);
    }
}