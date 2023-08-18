use crate::common::{Color, Resource, Tools, Coins, Scholars};
use crate::Result;

use crate::error::create_error;
use crate::race::Race;
pub struct Faction {
    race: Race,
    color: Color,
    // digging_cost: u32,
    // shipping_level: u32,
    // num_tools: Tools,
    // num_coins: Coins,
    // num_books: Books,
    // disc_track: [u32; 4],
    // power: PowerBowls,
}

impl Faction {
    pub fn new(race: &Race, color: &Color) -> Self {
        Faction {
            race: *race,
            color: *color,
        }
    }
}

trait IncomeTrack {
    fn new(color: Color) -> Self;
}

impl IncomeTrack for BuildingIncomeTrack<Tools> {
    fn new(color: Color) -> Self {
        // TODO: Income is currently unknown so this is a placeholder; fix when info is available
        Self {
            income_gain: vec![
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
                Tools::from(1),
            ],
            buildings_on_track: 9,
        }
    }
}

impl IncomeTrack for BuildingIncomeTrack<(Coins, Scholars)> {
    fn new(color: Color) -> Self {
        Self {
            income_gain: vec![
                Coins::from(2),
                Coins::from(2),
                Coins::from(2),
                Coins::from(2),
            ],
            buildings_on_track: 4,
        }
    }
}

pub struct BuildingIncomeTrack<T: Resource> {
    income_gain: Vec<T>, // The zeroth index is the amount gained when zero workshops are placed
    buildings_on_track: usize,
}

impl<T: Resource> BuildingIncomeTrack<T> {
    pub fn remove_building(&mut self) -> Result<()> {
        if self.buildings_on_track == 0 {
            Err(create_error("No workshops left on board"))
        } else {
            self.buildings_on_track -= 1;

            Ok(())
        }
    }

    pub fn put_building(&mut self) -> Result<()> {
        if self.buildings_on_track >= 9 {
            Err(create_error("Board is already full of workshops"))
        } else {
            self.buildings_on_track += 1;

            Ok(())
        }
    }

    pub fn income(&self) -> T {
        let num_income_slots = self.income_gain.len() - self.buildings_on_track;

        self.income_gain
            .iter()
            .take(num_income_slots)
            .fold(T::from(0), |a, b| a + *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn income_for_zero_workshops() {
        let track : BuildingIncomeTrack<Tools> = BuildingIncomeTrack::new(Color::Black); // Arbitrary color

        assert_eq!(track.income(), Tools(1));
    }

    #[test]
    fn income_for_two_workshops() -> Result<()> {
        let mut track: BuildingIncomeTrack<Tools> = BuildingIncomeTrack::new(Color::Blue); // Arbitrary color

        track.remove_building()?;
        track.remove_building()?;

        assert_eq!(track.income(), Tools(3));

        Ok(())
    }
}
