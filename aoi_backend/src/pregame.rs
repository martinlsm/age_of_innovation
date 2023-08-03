use itertools::izip;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    bonustile::BonusTile,
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

type FactionPool = Vec<(Race, BonusTile, TerrainType)>;

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

    let mut terrains: Vec<TerrainType> = enum_iterator::all().collect();
    terrains.shuffle(&mut rng);

    (
        izip!(races, bonus_tiles_pool, terrains).collect(),
        leftover_bonuses,
    )
}
