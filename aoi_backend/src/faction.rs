use std::cmp::min;

use crate::building::BuildingType;
use crate::common::{
    Books, Coins, Color, Discipline, Power, Resources, Scholars, Tools, DISCIPLINE_MAX,
};
use crate::Result;

use crate::error::create_error;
use crate::power::PowerBowls;
use crate::race::Race;

use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct Faction {
    race: Race,
    color: Color,
    digging_cost: Tools,
    sailing_level: u32,
    tools: Tools,
    coins: Coins,
    books: [Books; 4], // One book for each discipline, indexed by Discipline casted to usize
    scholars: Scholars,
    scholars_cap: Scholars,
    disc_track: [u32; 4],
    power: PowerBowls,
    dig_upg_cost: Resources,
    sailing_upg_cost: Resources,
}

impl Faction {
    pub fn new(race: &Race, color: &Color) -> Self {
        let mut faction = Faction {
            race: *race,
            color: *color,
            digging_cost: Tools(3),
            sailing_level: 0,
            tools: Tools(3),
            coins: Coins(15),
            scholars: Scholars(0),
            scholars_cap: Scholars(7),
            books: [Books(0), Books(0), Books(0), Books(0)],
            disc_track: [0, 0, 0, 0],
            power: PowerBowls::new(5, 7, 0),
            dig_upg_cost: Resources::from(Tools(1))
                + &Resources::from(Coins(5))
                + &Resources::from(Scholars(1)),
            sailing_upg_cost: Resources::from(Coins(4)) + &Resources::from(Scholars(1)),
        };

        faction.apply_race_bonus();
        faction.apply_color_bonus();

        faction
    }

    pub fn incr_disc(&mut self, disc: Discipline, amount: u32) -> u32 {
        let track: &mut u32 = &mut self.disc_track[disc as usize];
        *track = min(*track + amount, DISCIPLINE_MAX);
        *track
    }

    fn apply_race_bonus(&mut self) {
        match self.race {
            Race::Blessed => {
                self.incr_disc(Discipline::Banking, 1);
                self.incr_disc(Discipline::Law, 1);
                self.incr_disc(Discipline::Engineering, 1);
                self.incr_disc(Discipline::Medicine, 1);
                // TODO: Ability
            }
            Race::Monks => {
                self.incr_disc(Discipline::Law, 1);
                self.incr_disc(Discipline::Medicine, 1);

                self.tools += Tools(1);
                // TODO: Ability
            }
            Race::Felines => {
                self.incr_disc(Discipline::Banking, 1);
                self.incr_disc(Discipline::Medicine, 1);
                // TODO: Ability
            }
            Race::Navigators => {
                self.incr_disc(Discipline::Law, 3);
                // TODO: Ability
            }
            Race::Goblins => {
                self.incr_disc(Discipline::Banking, 1);
                self.incr_disc(Discipline::Engineering, 1);

                self.tools += Tools(1);
                // TODO: Ability
            }
            Race::Omar => {
                self.incr_disc(Discipline::Banking, 1);
                self.incr_disc(Discipline::Engineering, 1);
                // TODO: Ability
            }
            Race::Illusionists => {
                self.incr_disc(Discipline::Medicine, 2);
                // TODO: Ability
            }
            Race::Inventors => {
                // TODO: Ability
            }
            Race::Philosophers => {
                self.incr_disc(Discipline::Banking, 2);
                // TODO: Ability
            }
            Race::Lizards => {
                // TODO: Ability
            }
            Race::Psychics => {
                self.incr_disc(Discipline::Banking, 1);
                self.incr_disc(Discipline::Medicine, 1);

                self.tools += Tools(1);
                // TODO: Ability
            }
            Race::Moles => {
                self.incr_disc(Discipline::Engineering, 2);
                // TODO: Ability
            }
        }
    }

    fn apply_color_bonus(&mut self) {
        match self.color {
            Color::Yellow => {
                // TODO: Gain spade
            }
            Color::Brown => {
                self.dig_upg_cost = Resources::from(Tools(1))
                    + &Resources::from(Coins(1))
                    + &Resources::from(Scholars(1));
            }
            Color::Black => {
                self.scholars += Scholars(1);
                self.power = PowerBowls::new(0, 3, 9);
            }
            Color::Blue => {
                self.sailing_level = 1;
            }
            Color::Green => {
                self.incr_disc(Discipline::Banking, 1);
                self.incr_disc(Discipline::Law, 1);
                self.incr_disc(Discipline::Engineering, 1);
                self.incr_disc(Discipline::Medicine, 1);

                self.power = PowerBowls::new(4, 8, 0);
            }
            Color::Gray => {
                // TODO...
            }
            Color::Red => {
                // TODO...
            }
            Color::Colorless => {
                // For testing, no bonus
            }
        }
    }
}

pub struct BuildingIncomeTrack {
    income_gain: Vec<Resources>, // zeroth index is base income (not any buildings placed)
    num_occupied: usize,         // Number of occupied building slots on the income track
}

impl BuildingIncomeTrack {
    // TODO: Fill in the rest...
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
            (Color::Colorless, BuildingType::Workshop) => todo!(),
            (Color::Colorless, BuildingType::Guild) => todo!(),
            (Color::Colorless, BuildingType::School) => todo!(),
            (Color::Colorless, BuildingType::University) => todo!(),
            (Color::Colorless, BuildingType::Palace) => todo!(),
            (Color::Colorless, BuildingType::Tower) => todo!(),
            (Color::Colorless, BuildingType::Monument) => todo!(),
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
    use assert_json_diff::assert_json_include;

    use super::*;

    #[test]
    fn income_for_zero_workshops() {
        let track: BuildingIncomeTrack =
            BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop); // Arbitrary color

        assert_eq!(track.income(), Resources::from(Tools(1)));
    }

    #[test]
    fn income_for_two_workshops() {
        let mut track: BuildingIncomeTrack =
            BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop); // Arbitrary color

        track.remove_building().unwrap();
        track.remove_building().unwrap();

        assert_eq!(track.income(), Resources::from(Tools(3)));
    }

    #[test]
    fn remove_to_many_from_income_track() {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop);

        for _ in 0..9 {
            track.remove_building().unwrap();
        }

        assert!(track.remove_building().is_err());
    }

    #[test]
    fn put_building_on_track_when_full() {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &BuildingType::Workshop);

        assert!(track.put_building().is_err());
    }

    #[test]
    fn incr_disc() {
        let mut faction = Faction::new(&Race::Blessed, &Color::Yellow);

        let banking = faction.incr_disc(Discipline::Banking, 1);
        let law = faction.incr_disc(Discipline::Law, 2);
        let engineering = faction.incr_disc(Discipline::Engineering, 3);
        let medicine = faction.incr_disc(Discipline::Medicine, 4);

        assert_eq!(banking, 2); // Blessed starts with 1 in Banking (1 + 1)
        assert_eq!(law, 3); // Blessed starts with 1 in Law (1 + 2)
        assert_eq!(engineering, 4); // Blessed starts with 1 in Engineering (1 + 3)
        assert_eq!(medicine, 5); // Blessed starts with 1 in Medicine (1 + 4)
    }

    #[test]
    fn incr_disc_beyond_limit() {
        let mut faction = Faction::new(&Race::Monks, &Color::Black);

        let disc = faction.incr_disc(Discipline::Engineering, 15);

        // Discipline can't go over the hard limit of 12
        assert_eq!(disc, 12);
    }

    #[test]
    fn faction_blessed_has_correct_starting_state() {
        let blessed = Faction::new(&Race::Blessed, &Color::Colorless);
        let json = serde_json::to_value(&blessed).unwrap();

        let expected = json!({
            "race": "Blessed",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [1, 1, 1, 1],
            "power": [5, 7, 0],
            "dig_upg_cost": {
                "tools": 1,
                "coins": 5,
                "scholars": 1
            },
            "sailing_upg_cost": {
                "coins": 4,
                "scholars": 1
            },
        });

        assert_json_include!(actual: json, expected: expected);
    }
}
