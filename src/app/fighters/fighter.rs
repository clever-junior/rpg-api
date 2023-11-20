use crate::app::archetypes::energy::EnergyType;

struct FighterAttributes {
  life_points: u32,
  strength: u32,
  defense: u32,
  energy: Option<EnergyType>
}

pub trait Figther {
  fn attack(enemy: Self);

  fn special(enemy: Self);

  fn level_up();

  fn receive_damage(attack_point: u32) -> u32;
}