use crate::building::BuildingType;
use crate::common::{Color, Resources, Tools};
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

pub struct BuildingIncomeTrack {
    income_gain: Vec<Resources>, // zeroth index is base income (not any buildings placed)
    num_occupied: usize,         // Number of occupied building slots on the income track
}

impl BuildingIncomeTrack {
    pub fn new(color: &Color, building: &BuildingType) -> Self {
        match (color, building) {
            (Color::Yellow, BuildingType::Workshop) => Self {
                income_gain: vec![
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                ],
                num_occupied: 9,
            },
            (Color::Yellow, BuildingType::Guild) => todo!(),
            (Color::Yellow, BuildingType::School) => todo!(),
            (Color::Yellow, BuildingType::University) => todo!(),
            (Color::Yellow, BuildingType::Palace) => todo!(),
            (Color::Yellow, BuildingType::Tower) => todo!(),
            (Color::Yellow, BuildingType::Monument) => todo!(),
            (Color::Brown, BuildingType::Workshop) => todo!(),
            (Color::Brown, BuildingType::Guild) => todo!(),
            (Color::Brown, BuildingType::School) => todo!(),
            (Color::Brown, BuildingType::University) => todo!(),
            (Color::Brown, BuildingType::Palace) => todo!(),
            (Color::Brown, BuildingType::Tower) => todo!(),
            (Color::Brown, BuildingType::Monument) => todo!(),
            (Color::Black, BuildingType::Workshop) => todo!(),
            (Color::Black, BuildingType::Guild) => todo!(),
            (Color::Black, BuildingType::School) => todo!(),
            (Color::Black, BuildingType::University) => todo!(),
            (Color::Black, BuildingType::Palace) => todo!(),
            (Color::Black, BuildingType::Tower) => todo!(),
            (Color::Black, BuildingType::Monument) => todo!(),
            (Color::Blue, BuildingType::Workshop) => todo!(),
            (Color::Blue, BuildingType::Guild) => todo!(),
            (Color::Blue, BuildingType::School) => todo!(),
            (Color::Blue, BuildingType::University) => todo!(),
            (Color::Blue, BuildingType::Palace) => todo!(),
            (Color::Blue, BuildingType::Tower) => todo!(),
            (Color::Blue, BuildingType::Monument) => todo!(),
            (Color::Green, BuildingType::Workshop) => todo!(),
            (Color::Green, BuildingType::Guild) => todo!(),
            (Color::Green, BuildingType::School) => todo!(),
            (Color::Green, BuildingType::University) => todo!(),
            (Color::Green, BuildingType::Palace) => todo!(),
            (Color::Green, BuildingType::Tower) => todo!(),
            (Color::Green, BuildingType::Monument) => todo!(),
            (Color::Gray, BuildingType::Workshop) => todo!(),
            (Color::Gray, BuildingType::Guild) => todo!(),
            (Color::Gray, BuildingType::School) => todo!(),
            (Color::Gray, BuildingType::University) => todo!(),
            (Color::Gray, BuildingType::Palace) => todo!(),
            (Color::Gray, BuildingType::Tower) => todo!(),
            (Color::Gray, BuildingType::Monument) => todo!(),
            (Color::Red, BuildingType::Workshop) => todo!(),
            (Color::Red, BuildingType::Guild) => todo!(),
            (Color::Red, BuildingType::School) => todo!(),
            (Color::Red, BuildingType::University) => todo!(),
            (Color::Red, BuildingType::Palace) => todo!(),
            (Color::Red, BuildingType::Tower) => todo!(),
            (Color::Red, BuildingType::Monument) => todo!(),
        }
    }

    pub fn remove_building(&mut self) -> Result<()> {
        if self.num_occupied == 0 {
            Err(create_error("No workshops left on board"))
        } else {
            self.num_occupied -= 1;

            Ok(())
        }
    }

    pub fn put_building(&mut self) -> Result<()> {
        if self.num_occupied >= 9 {
            Err(create_error("Board is already full of workshops"))
        } else {
            self.num_occupied += 1;

            Ok(())
        }
    }

    pub fn income(&self) -> Resources {
        let num_income_slots = self.income_gain.len() - self.num_occupied;

        self.income_gain
            .iter()
            .take(num_income_slots)
            .fold(Resources::new(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn income_for_zero_workshops() {
        let track: BuildingIncomeTrack =
            BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop); // Arbitrary color

        assert_eq!(track.income(), Resources::from(Tools(1)));
    }

    #[test]
    fn income_for_two_workshops() -> Result<()> {
        let mut track: BuildingIncomeTrack =
            BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop); // Arbitrary color

        track.remove_building()?;
        track.remove_building()?;

        assert_eq!(track.income(), Resources::from(Tools(3)));

        Ok(())
    }

    #[test]
    fn remove_to_many_from_income_track() -> Result<()> {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop);

        for _ in 0..9 {
            track.remove_building()?;
        }

        assert!(track.remove_building().is_err());

        Ok(())
    }

    #[test]
    fn put_building_on_track_when_full() -> Result<()> {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop);

        assert!(track.put_building().is_err());

       Ok(())
    }
}
