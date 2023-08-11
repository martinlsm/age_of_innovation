use std::fs::create_dir;

use crate::common::{Color, Tools};
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

pub struct WorkshopIncomeTrack<T> {
    income_gain: [T; 10], // The zeroth index is the amount gained when zero workshops are placed
    workshops_on_track: usize,
}

impl WorkshopIncomeTrack<T> {
    pub fn new(color: Color) -> Self {
        WorkshopIncomeTrack {
            // TODO: Income is currently unknown so this is a placeholder; fix when info is available
            income_gain: [
                T(1),
                T(1),
                T(1),
                T(1),
                T(1),
                T(1),
                T(1),
                T(1),
                T(1),
                T(1),
            ],
            workshops_on_track: 9,
        }
    }

    pub fn remove_workshop(&mut self) -> Result<()> {
        if self.workshops_on_track == 0 {
            Err(create_error("No workshops left on board"))
        } else {
            self.workshops_on_track -= 1;

            Ok(())
        }

    }

    pub fn put_workshop(&mut self) -> Result<()> {
        if self.workshops_on_track >= 9 {
            Err(create_error("Board is already full of workshops"))
        } else {
            self.workshops_on_track += 1;

            Ok(())
        }
    }

    pub fn income(&self) -> Tools {
        let num_income_slots = self.income_gain.len() - self.workshops_on_track;

        self.income_gain.iter().take(num_income_slots).fold(Tools(0), |a, b| Tools(a.0 + b.0))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn income_for_zero_workshops() {
        let track : WorkshopIncomeTrack<Tools> = WorkshopIncomeTrack::new(Color::Black); // Arbitrary color

        assert_eq!(track.income(), Tools(1));
    }

    #[test]
    fn income_for_two_workshops() -> Result<()> {
        let mut track = WorkshopIncomeTrack::new(Color::Blue); // Arbitrary color

        track.remove_workshop()?;
        track.remove_workshop()?;

        assert_eq!(track.income(), Tools(3));

        Ok(())
    }
}