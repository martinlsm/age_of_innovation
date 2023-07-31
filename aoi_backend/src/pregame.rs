use crate::{gamemap, scoringtile};

pub struct PreGame {
    num_players: u32,
    map: Vec<Vec<gamemap::Hex>>,
    scoring_tiles: Vec<scoringtile::ScoringTile>,
}

impl PreGame {
    pub fn new_random(num_players: u32) -> Self {
        PreGame {
            num_players: num_players,
            map: gamemap::open_map(),
            scoring_tiles: scoringtile::new_game_random_tiles(),
        }
    }
}
