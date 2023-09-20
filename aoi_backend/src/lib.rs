// Public modules
pub mod bonustile;
pub mod bookaction;
pub mod building;
pub mod common;
pub mod error;
pub mod faction;
pub mod game;
pub mod gamephase;
pub mod helpers;
pub mod map;
pub mod power;
pub mod pregame;
pub mod race;
pub mod resources;
pub mod scoringtile;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
