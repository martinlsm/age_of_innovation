use std::rc::Rc;

use itertools::izip;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    bonustile::BonusTile,
    bookaction::{new_game_random_book_actions, BookAction},
    common::Color,
    error::create_error,
    faction::Faction,
    gamemap,
    race::Race,
    scoringtile, Result,
};

pub struct PreGame {
    num_players: u32,
    map: Vec<Vec<gamemap::Hex>>,
    scoring_tiles: Vec<scoringtile::ScoringTile>,
    faction_pool: Rc<FactionPool>,
    leftover_bonuses: Vec<BonusTile>,
    book_actions: Vec<BookAction>,
}

type FactionPool = Vec<(Race, BonusTile, Color)>;

impl PreGame {
    pub fn new_random(num_players: u32) -> Self {
        let (faction_pool, leftover_bonuses) = gen_random_faction_pool();

        PreGame {
            num_players,
            map: gamemap::open_map(),
            scoring_tiles: scoringtile::new_game_random_tiles(),
            faction_pool: Rc::new(faction_pool),
            leftover_bonuses,
            book_actions: new_game_random_book_actions(),
        }
    }
}

pub struct FactionSelector {
    selected: Vec<usize>,
    faction_pool: Rc<FactionPool>,
    num_players: u32,
}

impl FactionSelector {
    pub fn new(pregame: &PreGame) -> Self {
        FactionSelector {
            selected: Vec::new(),
            faction_pool: pregame.faction_pool.clone(),
            num_players: pregame.num_players,
        }
    }

    pub fn select(&mut self, idx: usize) -> Result<()> {
        if self.selected.len() >= self.num_players as usize {
            return Err(create_error("All factions are already selected"));
        }
        if self.selected.contains(&idx) {
            return Err(create_error("Faction has already been selected"));
        }

        self.selected.push(idx);

        Ok(())
    }

    pub fn finish(&self) -> Result<Vec<Faction>> {
        if self.selected.len() != self.num_players as usize {
            return Err(create_error("All players must have selected a faction"));
        }

        let mut res = Vec::new();
        for idx in &self.selected {
            // TODO: Deal with the bonus tile
            let (race, _, color) = &(*self.faction_pool)[*idx];
            res.push(Faction::new(race, color));
        }

        Ok(res)
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

    #[test]
    fn selected_factions_are_correct_in_number() {
        for num_players in 2..5 {
            let pregame = PreGame::new_random(num_players);
            let mut selector = FactionSelector::new(&pregame);

            for i in 0..num_players {
                selector.select(i as usize).unwrap();
            }

            let selected = selector.finish().unwrap();
            assert_eq!(selected.len(), num_players as usize);
        }
    }

    #[test]
    fn finish_faction_selection_prematurely() {
        for num_players in 2..5 {
            let pregame = PreGame::new_random(num_players);
            let mut selector = FactionSelector::new(&pregame);

            // Select faction to all players except the two last ones
            for i in 0..(num_players - 2) {
                selector.select(i as usize).unwrap();
            }

            assert!(selector.finish().is_err());
        }
    }

    #[test]
    fn select_too_many_factions() {
        for num_players in 2..5 {
            let pregame = PreGame::new_random(num_players);
            let mut selector = FactionSelector::new(&pregame);

            for i in 0..num_players {
                selector.select(i as usize).unwrap();
            }

            assert!(selector.select(num_players as usize).is_err());
        }
    }

    #[test]
    fn select_duplicate_faction() {
        let pregame = PreGame::new_random(4);
        let mut selector = FactionSelector::new(&pregame);

        selector.select(0).unwrap();

        assert!(selector.select(0).is_err());
    }
}
