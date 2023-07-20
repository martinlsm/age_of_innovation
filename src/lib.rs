// Public modules
pub mod error;
pub mod gamemap;

// Private modules

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
