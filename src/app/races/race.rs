
pub struct Race {
  pub name: String,
  pub dexterity: u32,
}

pub trait RaceMethods {
  fn new(name: String, dexterity: u32) -> Self;

  fn get_name(&self) -> &str;

  fn get_dexterity(&self) -> u32;

  fn get_max_life_points(&self) -> u32;

  fn created_races_intances(&self) -> u32 {
      unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct SomeRace {
      race: Race,
      max_life_points: u32,
  }

  impl RaceMethods for SomeRace {
      fn new(name: String, dexterity: u32) -> Self {
          Self {
              race: Race { name, dexterity },
              max_life_points: 10,
          }
      }

      fn get_name(&self) -> &str {
          &self.race.name
      }

      fn get_dexterity(&self) -> u32 {
          self.race.dexterity
      }

      fn get_max_life_points(&self) -> u32 {
          self.max_life_points
      }
  }

  #[test]
  fn test_race_attr() {
      let race = Race {
          name: String::from("Name"),
          dexterity: 0,
      };

      assert!(race.name.len() > 0);
      assert_eq!(race.name, String::from("Name"));
      assert_eq!(race.dexterity, 0);
  }

  #[test]
  fn test_race_impl() {
      let some_race = SomeRace::new(String::from("Name"), 0);

      assert_eq!(some_race.get_name(), String::from("Name"));
      assert_eq!(some_race.get_dexterity(), 0);
      assert_eq!(some_race.get_max_life_points(), 10);
  }
}