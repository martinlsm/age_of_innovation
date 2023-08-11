use crate::gamemap::Terrain;

use crate::race::Race;
pub struct Faction {
    race: Race,
    color: Terrain,
}

impl Faction {
    pub fn get_color(&self) -> &Terrain {
        &self.color
    }
}
