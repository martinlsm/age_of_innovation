// Public modules
pub mod building;
pub mod common;
pub mod error;
pub mod faction;
pub mod gamemap;
pub mod power;
pub mod resources;
pub mod pregame;
pub mod race;
pub mod scoringtile;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
