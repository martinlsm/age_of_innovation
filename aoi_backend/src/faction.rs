use crate::common::Color;

use crate::race::Race;
pub struct Faction {
    race: Race,
    color: Color,
}

impl Faction {
    pub fn new(race: &Race, color: &Color) -> Self {
        Faction {
            race: *race,
            color: *color,
        }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}
