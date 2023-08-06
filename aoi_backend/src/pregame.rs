use itertools::izip;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    bonustile::BonusTile,
    common::Color,
    gamemap::{self, TerrainType},
    race::Race,
    scoringtile,
};

pub struct PreGame {
    num_players: u32,
    map: Vec<Vec<gamemap::Hex>>,
    scoring_tiles: Vec<scoringtile::ScoringTile>,
    faction_pool: FactionPool,
    leftover_bonuses: Vec<BonusTile>,
}

type FactionPool = Vec<(Race, BonusTile, Color)>;

impl PreGame {
    pub fn new_random(num_players: u32) -> Self {
        let (faction_pool, leftover_bonuses) = gen_random_faction_pool();

        PreGame {
            num_players,
            map: gamemap::open_map(),
            scoring_tiles: scoringtile::new_game_random_tiles(),
            faction_pool,
            leftover_bonuses,
        }
    }
}

fn gen_random_faction_pool() -> (FactionPool, Vec<BonusTile>) {
    let mut rng = thread_rng();

    let mut races: Vec<Race> = enum_iterator::all().collect();
    races.shuffle(&mut rng);

    let mut bonus_tiles: Vec<BonusTile> = enum_iterator::all().collect();
    bonus_tiles.shuffle(&mut rng);
    let bonus_tiles_pool: Vec<BonusTile> = bonus_tiles.as_slice()[..7].to_vec();
    let leftover_bonuses: Vec<BonusTile> = bonus_tiles.as_slice()[7..10].to_vec();

    let mut colors: Vec<Color> = enum_iterator::all().collect();
    colors.shuffle(&mut rng);

    (
        izip!(races, bonus_tiles_pool, colors).collect(),
        leftover_bonuses,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leftover_bonuses_are_3() {
        for num_players in 2..5 {
            let pregame = PreGame::new_random(num_players);

            assert_eq!(pregame.leftover_bonuses.len(), 3);
        }
    }

    #[test]
    fn there_are_7_possible_faction_selections() {
        for num_players in 2..5 {
            let pregame = PreGame::new_random(num_players);

            assert_eq!(pregame.faction_pool.len(), 7);
        }
    }

    #[test]
    fn faction_pool_have_no_duplicate_entries() {
        for num_players in 2..5 {
            let pregame = PreGame::new_random(num_players);
            let v = &pregame.faction_pool;

            // Check for duplicate races
            assert!(v
                .iter()
                .all(|a| v.iter().filter(|&b| a.0 == b.0).count() == 1));
            // Check for duplicate bonus tiles
            assert!(v
                .iter()
                .all(|a| v.iter().filter(|&b| a.1 == b.1).count() == 1));
            // Check for duplicate colors
            assert!(v
                .iter()
                .all(|a| v.iter().filter(|&b| a.2 == b.2).count() == 1));
        }
    }
}
