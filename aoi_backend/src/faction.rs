use crate::gamemap::TerrainType;

use crate::race::Race;
pub struct Faction {
    race: Race,
    color: TerrainType,
}

impl Faction {
    pub fn get_color(&self) -> &TerrainType {
        &self.color
    }
}
