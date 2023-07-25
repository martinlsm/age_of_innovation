// Public modules
pub mod error;
pub mod gamemap;
pub mod power;
pub mod resources;

// Private modules

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
