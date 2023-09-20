use std::rc::Rc;

use crate::{bonustile::BonusTile, common::Color, map::{self, Pos}, race::Race, building::Building, error, Result, resources::Power};

type FactionPool = Vec<(Race, BonusTile, Color)>;

pub type PlayerId = usize;

pub struct Data {
    pub num_players: u32,
    pub map: Vec<Vec<map::Hex>>,
    pub faction_pool: Rc<FactionPool>,
    pub leftover_bonuses: Vec<BonusTile>,
}

pub struct Game {
    pub phase: Box<dyn Phase>,
    state: PendingState,
    data: Data,
}

pub enum PendingState {
    PlaceInitialBuilding(PlayerId),
    // TODO: PerformAction(PlayerId),
    // TODO: GainPower(PlayerId, Power),
    // TODO: SpendSpade(PlayerId, u32),
    // TODO: FoundTown(PlayerId, Vec<Pos>),
}

pub enum Action {
    BuildWorkshop(Pos),
    UpgradeToGuild(Pos),
    // TODO ...
}

pub trait Phase {
    fn player_to_move(&self) -> Result<PlayerId> {
        Err(error::create_error("No player can make a move right now"))
    }
    fn place_building(&self, _player_id: PlayerId, _pos: Pos) -> Result<Building> {
        Err(error::create_error("Unable to place building at this time"))
    }
    //fn do_action(&self, _player_id: PlayerId, action: Action) -> Result<> {

    //}
}