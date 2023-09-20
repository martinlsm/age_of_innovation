use std::cmp::min;

use crate::building::Building;
use crate::common::{Color, Discipline, DISCIPLINE_MAX};
use crate::Result;

use crate::error::create_error;
use crate::power::PowerBowls;
use crate::race::Race;
use crate::resources::{Books, Coins, Power, Resources, Scholars, Tools};

use serde::Serialize;

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

    pub fn get_color(&self) -> Color {
        self.color
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
            Race::Raceless => {
                // For testing, no bonus
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
                self.power = PowerBowls::new(3, 9, 0);
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
    pub fn new(color: &Color, building: &Building) -> Result<Self> {
        match (color, building) {
            (Color::Gray, Building::Workshop) => Ok(BuildingIncomeTrack {
                income_gain: vec![
                    Resources::from(Tools(1)) + &Resources::from(Coins(2)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(0)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                ],
                num_occupied: 9,
            }),
            (_, Building::Workshop) => Ok(BuildingIncomeTrack {
                income_gain: vec![
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(0)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                    Resources::from(Tools(1)),
                ],
                num_occupied: 9,
            }),
            (_, Building::Guild) => Ok(BuildingIncomeTrack {
                income_gain: vec![
                    Resources::none(),
                    Resources::from(Coins(2)) + &Resources::from(Power(1)),
                    Resources::from(Coins(2)) + &Resources::from(Power(1)),
                    Resources::from(Coins(2)) + &Resources::from(Power(2)),
                    Resources::from(Coins(2)) + &Resources::from(Power(2)),
                ],
                num_occupied: 4,
            }),
            (_, Building::School) => Ok(BuildingIncomeTrack {
                income_gain: vec![
                    Resources::none(),
                    Resources::from(Scholars(1)),
                    Resources::from(Scholars(1)),
                    Resources::from(Scholars(1)),
                ],
                num_occupied: 3,
            }),
            (_, Building::University) => Ok(BuildingIncomeTrack {
                income_gain: vec![Resources::none(), Resources::from(Scholars(1))],
                num_occupied: 1,
            }),
            _ => Err(create_error("No income track for specified building")),
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
            .fold(Resources::none(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;

    #[test]
    fn income_for_zero_workshops() {
        let track = BuildingIncomeTrack::new(&Color::Yellow, &Building::Workshop).unwrap(); // Arbitrary color

        assert_eq!(track.income(), Resources::from(Tools(1)));
    }

    #[test]
    fn income_for_two_workshops() {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &Building::Workshop).unwrap(); // Arbitrary color

        track.remove_building().unwrap();
        track.remove_building().unwrap();

        assert_eq!(track.income(), Resources::from(Tools(3)));
    }

    #[test]
    fn income_for_nine_workshops() {
        let mut track = BuildingIncomeTrack::new(&Color::Black, &Building::Workshop).unwrap();

        for _ in 0..9 {
            track.remove_building().unwrap();
        }

        assert_eq!(track.income(), Resources::from(Tools(9)));
    }

    #[test]
    fn gray_has_extra_gold_income() {
        let mut track = BuildingIncomeTrack::new(&Color::Gray, &Building::Workshop).unwrap();

        track.remove_building().unwrap();

        assert_eq!(
            track.income(),
            Resources::from(Tools(2)) + &Resources::from(Coins(2))
        );
    }

    #[test]
    fn remove_to_many_from_income_track() {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &Building::Workshop).unwrap();

        for _ in 0..9 {
            track.remove_building().unwrap();
        }

        assert!(track.remove_building().is_err());
    }

    #[test]
    fn put_building_on_track_when_full() {
        let mut track = BuildingIncomeTrack::new(&Color::Yellow, &Building::Workshop).unwrap();

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

    #[test]
    fn faction_monks_has_correct_starting_state() {
        let monks = Faction::new(&Race::Monks, &Color::Colorless);
        let json = serde_json::to_value(&monks).unwrap();

        let expected = json!({
            "race": "Monks",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 4,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 1, 0, 1],
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

    #[test]
    fn faction_felines_has_correct_starting_state() {
        let felines = Faction::new(&Race::Felines, &Color::Colorless);
        let json = serde_json::to_value(&felines).unwrap();

        let expected = json!({
            "race": "Felines",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [1, 0, 0, 1],
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

    #[test]
    fn faction_navigators_has_correct_starting_state() {
        let navigators = Faction::new(&Race::Navigators, &Color::Colorless);
        let json = serde_json::to_value(&navigators).unwrap();

        let expected = json!({
            "race": "Navigators",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 3, 0, 0],
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

    #[test]
    fn faction_goblins_has_correct_starting_state() {
        let goblins = Faction::new(&Race::Goblins, &Color::Colorless);
        let json = serde_json::to_value(&goblins).unwrap();

        let expected = json!({
            "race": "Goblins",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 4,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [1, 0, 1, 0],
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

    #[test]
    fn faction_omar_has_correct_starting_state() {
        let omar = Faction::new(&Race::Omar, &Color::Colorless);
        let json = serde_json::to_value(&omar).unwrap();

        let expected = json!({
            "race": "Omar",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [1, 0, 1, 0],
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

    #[test]
    fn faction_illusionists_has_correct_starting_state() {
        let illusionists = Faction::new(&Race::Illusionists, &Color::Colorless);
        let json = serde_json::to_value(&illusionists).unwrap();

        let expected = json!({
            "race": "Illusionists",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 0, 0, 2],
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

    #[test]
    fn faction_inventors_has_correct_starting_state() {
        let inventors = Faction::new(&Race::Inventors, &Color::Colorless);
        let json = serde_json::to_value(&inventors).unwrap();

        let expected = json!({
            "race": "Inventors",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 0, 0, 0],
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

    #[test]
    fn faction_philosophers_has_correct_starting_state() {
        let philosophers = Faction::new(&Race::Philosophers, &Color::Colorless);
        let json = serde_json::to_value(&philosophers).unwrap();

        let expected = json!({
            "race": "Philosophers",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [2, 0, 0, 0],
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

    #[test]
    fn faction_lizards_has_correct_starting_state() {
        let lizards = Faction::new(&Race::Lizards, &Color::Colorless);
        let json = serde_json::to_value(&lizards).unwrap();

        let expected = json!({
            "race": "Lizards",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 0, 0, 0],
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

    #[test]
    fn faction_psychics_has_correct_starting_state() {
        let psychics = Faction::new(&Race::Psychics, &Color::Colorless);
        let json = serde_json::to_value(&psychics).unwrap();

        let expected = json!({
            "race": "Psychics",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 4,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [1, 0, 0, 1],
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

    #[test]
    fn faction_moles_has_correct_starting_state() {
        let moles = Faction::new(&Race::Moles, &Color::Colorless);
        let json = serde_json::to_value(&moles).unwrap();

        let expected = json!({
            "race": "Moles",
            "color": "Colorless",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 0, 2, 0],
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

    #[test]
    fn yellow_has_correct_starting_state() {
        let faction = Faction::new(&Race::Raceless, &Color::Yellow);
        let json = serde_json::to_value(&faction).unwrap();

        let expected = json!({
            "race": "Raceless",
            "color": "Yellow",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 0, 0, 0],
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

    #[test]
    fn brown_has_correct_starting_state() {
        let faction = Faction::new(&Race::Raceless, &Color::Brown);
        let json = serde_json::to_value(&faction).unwrap();

        let expected = json!({
            "race": "Raceless",
            "color": "Brown",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 0,
            "scholars_cap": 7,
            "disc_track": [0, 0, 0, 0],
            "power": [5, 7, 0],
            "dig_upg_cost": {
                "tools": 1,
                "coins": 1,
                "scholars": 1
            },
            "sailing_upg_cost": {
                "coins": 4,
                "scholars": 1
            },
        });
        assert_json_include!(actual: json, expected: expected);
    }

    #[test]
    fn black_has_correct_starting_state() {
        let faction = Faction::new(&Race::Raceless, &Color::Black);
        let json = serde_json::to_value(&faction).unwrap();

        let expected = json!({
            "race": "Raceless",
            "color": "Black",
            "digging_cost": 3,
            "sailing_level": 0,
            "tools": 3,
            "coins": 15,
            "books": [0, 0, 0, 0],
            "scholars": 1,
            "scholars_cap": 7,
            "disc_track": [0, 0, 0, 0],
            "power": [3, 9, 0],
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
