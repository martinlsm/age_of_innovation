use crate::common::{Discipline, VP};

use itertools::Itertools;
use rand;
use rand::prelude::SliceRandom;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct ScoringTile {
    pub left_side: LeftSide,
    pub right_side: RightSide,
    id: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct LeftSide {
    pub goal: LeftSideGoal,
    pub amount: VP,
    _private: (), // This will hinder other modules to create new instances of this struct.
}

#[derive(Clone, Copy, PartialEq)]
pub enum LeftSideGoal {
    BuildWorkshop,
    BuildGuild,
    BuildSchool,
    BuildBigBuilding,
    Dig,
    AdvanceDiscipline,
    FoundCity,
    AdvanceSailingOrDigging,
    GainInnovationTile,
}

#[derive(Clone, Copy, PartialEq)]
pub struct RightSide {
    pub rew: RightSideReward,
    pub rew_amount: u32,
    pub disc: Discipline,
    pub disc_requirement: u32,
    _private: (), // This will hinder other modules to create new instances of this struct.
}

#[derive(Clone, Copy, PartialEq)]
pub enum RightSideReward {
    Books,
    Coins,
    Tools,
    Power,
    Scholars,
    Spades,
}

impl Hash for ScoringTile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for ScoringTile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ScoringTile {}

pub fn new_game_random_tiles() -> Vec<ScoringTile> {
    let mut rng = rand::thread_rng();

    let mut tile_pool = all_scoring_tiles();
    tile_pool.shuffle(&mut rng);

    let mut r56_scoring_tiles = tile_pool
        .iter()
        .filter(|&tile| tile.left_side.goal != LeftSideGoal::Dig)
        .take(2)
        .copied()
        .collect_vec();

    let r14_scoring_tiles = tile_pool
        .iter()
        .filter(|tile| !r56_scoring_tiles.contains(&tile))
        .take(4)
        .copied()
        .collect_vec();

    let mut res = r14_scoring_tiles;
    res.append(&mut r56_scoring_tiles);

    res
}

fn all_scoring_tiles() -> Vec<ScoringTile> {
    let mut id_gen = 0..;
    vec![
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildWorkshop,
                amount: VP(2),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Scholars,
                rew_amount: 1,
                disc: Discipline::Law,
                disc_requirement: 4,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildWorkshop,
                amount: VP(2),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Power,
                rew_amount: 4,
                disc: Discipline::Banking,
                disc_requirement: 3,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildGuild,
                amount: VP(3),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Books,
                rew_amount: 1,
                disc: Discipline::Law,
                disc_requirement: 3,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildGuild,
                amount: VP(3),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Spades,
                rew_amount: 1,
                disc: Discipline::Medicine,
                disc_requirement: 4,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildSchool,
                amount: VP(4),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Coins,
                rew_amount: 1,
                disc: Discipline::Banking,
                disc_requirement: 1,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildBigBuilding,
                amount: VP(5),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Tools,
                rew_amount: 1,
                disc: Discipline::Medicine,
                disc_requirement: 2,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::BuildBigBuilding,
                amount: VP(5),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Tools,
                rew_amount: 1,
                disc: Discipline::Banking,
                disc_requirement: 2,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::Dig,
                amount: VP(2),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Coins,
                rew_amount: 1,
                disc: Discipline::Engineering,
                disc_requirement: 1,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::AdvanceDiscipline,
                amount: VP(1),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Books,
                rew_amount: 1,
                disc: Discipline::Medicine,
                disc_requirement: 3,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::FoundCity,
                amount: VP(5),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Spades,
                rew_amount: 1,
                disc: Discipline::Engineering,
                disc_requirement: 4,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::AdvanceSailingOrDigging,
                amount: VP(3),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Scholars,
                rew_amount: 1,
                disc: Discipline::Engineering,
                disc_requirement: 3,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
        ScoringTile {
            left_side: LeftSide {
                goal: LeftSideGoal::GainInnovationTile,
                amount: VP(5),
                _private: (),
            },
            right_side: RightSide {
                rew: RightSideReward::Power,
                rew_amount: 3,
                disc: Discipline::Law,
                disc_requirement: 2,
                _private: (),
            },
            id: id_gen.next().unwrap(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    #[test]
    fn scoring_tiles_for_new_game_are_6() {
        for _ in 0..100 {
            // Run multiple times since randomness is involved.
            let tiles = new_game_random_tiles();
            assert_eq!(tiles.len(), 6);
        }
    }

    #[test]
    fn scoring_tiles_no_duplicates() {
        for _ in 0..100 {
            // Run multiple times since randomness is involved.
            let tiles = new_game_random_tiles();
            let mut set = HashSet::new();
            tiles.into_iter().all(|x| set.insert(x));

            assert_eq!(set.len(), 6);
        }
    }
}
