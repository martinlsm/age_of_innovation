use crate::error::create_error;
use crate::faction::Faction;
use crate::{map, Result};

use crate::common::Color;

use crate::gamephase::incomephase::IncomePhase;
use crate::gamephase::PlayerId;

pub struct BuildingPlacer {
    factions: Vec<Faction>, // Indices map to player IDs
    map: map::Map,
    placed: Vec<(PlayerId, map::Pos)>,
}

impl BuildingPlacer {
    pub fn new(factions: Vec<Faction>, map: map::Map) -> BuildingPlacer {
        BuildingPlacer {
            factions,
            map: map,
            placed: Vec::new(),
        }
    }

    pub fn place(&mut self, player_id: PlayerId, pos: map::Pos) -> Result<()> {
        // Colorless factions are allowed to place anywhere for the purposes of testing.
        if !self.valid_placement(player_id, pos) {
            return Err(create_error("Not a valid hex"));
        }

        self.placed.push((player_id, pos));

        Ok(())
    }

    pub fn valid_placement(&self, player_id: PlayerId, pos: map::Pos) -> bool {
        let inside_bounds = pos.0 < map::MAP_HEIGHT && pos.1 < map::MAP_WIDTH;
        if !inside_bounds {
            return false;
        }

        let already_occupied = self.placed.iter().filter(|(_, p)| pos == *p).count() == 1;
        if already_occupied {
            return false;
        }

        let this_players_turn = self
            .player_to_move()
            .map(|p| player_id == p)
            .unwrap_or(false);
        if !this_players_turn {
            return false;
        }

        let player_color = self.factions[player_id].get_color();
        let valid_color: bool = match self.map[pos.0][pos.1].terrain {
            map::Terrain::Land(hex_color) => {
                if player_color == Color::Colorless {
                    // Colorless factions are allowed to place anywhere for the purposes of testing.
                    true
                } else {
                    hex_color == player_color
                }
            }
            map::Terrain::Water => false,
        };

        valid_color
    }

    pub fn player_to_move(&self) -> Result<PlayerId> {
        let num_players = self.factions.len();
        if self.placed.len() == num_players * 2 {
            Err(create_error("All buildings have already been placed"))
        } else if self.placed.len() < num_players {
            Ok(self.placed.len())
        } else {
            Ok(2 * num_players - self.placed.len() - 1)
        }
    }

    pub fn finish(self) -> Result<IncomePhase> {
        if self.placed.len() == 2 * self.factions.len() {
            Ok(IncomePhase {})
        } else {
            Err(create_error(
                "All players have not placed all their buildings",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{common::Color, race::Race};

    use std::iter::zip;

    use itertools::Itertools;
    use parameterized::parameterized;

    fn create_colorless_factions(num_players: usize) -> Vec<Faction> {
        let mut factions = Vec::new();
        for _ in 0..num_players {
            factions.push(Faction::new(&Race::Raceless, &Color::Colorless));
        }

        factions
    }

    fn create_test_building_placer(num_players: usize) -> BuildingPlacer {
        let factions = create_colorless_factions(num_players);
        let map = map::open_map(map::MapId::Debug);

        BuildingPlacer::new(factions, map)
    }

    fn valid_place_order_debug_map(num_players: usize) -> (Vec<PlayerId>, Vec<map::Pos>) {
        assert!((0..=5).contains(&num_players));

        let placement_order = (0..num_players).chain((0..num_players).rev()).collect_vec();
        let hex_order = vec![
            (1, 0), // Yellow
            (2, 0), // Brown
            (3, 0), // Black
            // (4, 0) is water
            (5, 0), // Blue
            (6, 0), // Green
            (7, 0), // Gray
            (8, 0), // Red
            (1, 1), // Yellow
            (2, 1), // Brown
            (3, 1), // Black
            // (4, 1) is water
            (5, 1), // Blue
            (6, 1), // Green
            (7, 1), // Gray
            (8, 1), // Red
        ];

        (placement_order, hex_order)
    }

    #[parameterized(num_players = { 2, 3, 4, 5 })]
    fn place_all_buildings_in_valid_order(num_players: usize) {
        let mut placer: BuildingPlacer = create_test_building_placer(num_players);
        let (placement_order, hex_order) = valid_place_order_debug_map(num_players);

        for (player_id, pos) in zip(placement_order, hex_order).take(num_players * 2) {
            placer.place(player_id as PlayerId, pos).unwrap();
        }

        assert!(placer.finish().is_ok());
    }

    #[parameterized(num_players = { 2, 3, 4, 5 })]
    fn first_player_only_one_who_may_start_placing(num_players: usize) {
        let mut placer: BuildingPlacer = create_test_building_placer(num_players);

        assert_eq!(placer.player_to_move().unwrap(), 0);
        for player_id in 1..num_players as PlayerId {
            assert!(placer.place(player_id, (1, 0)).is_err());
        }
        assert_eq!(placer.player_to_move().unwrap(), 0);
    }

    #[parameterized(num_players = { 2, 3, 4, 5 })]
    fn player_to_move_is_correct(num_players: usize) {
        let mut placer: BuildingPlacer = create_test_building_placer(num_players);

        let (placement_order, hex_order) = valid_place_order_debug_map(num_players);

        for (player_id, pos) in zip(placement_order.clone(), hex_order) {
            assert_eq!(placer.player_to_move().unwrap(), player_id);
            placer.place(player_id, pos).unwrap();
        }
        assert!(placer.player_to_move().is_err());
    }

    #[parameterized(color = { Color::Yellow, Color::Brown, Color::Black, Color::Blue, Color::Green, Color::Gray, Color::Red })]
    fn only_allowed_to_place_on_native_color(color: Color) {
        // Let player ID = 0 have the color to test
        let player_id: PlayerId = 0;
        let factions = vec![
            Faction::new(&Race::Raceless, &color),
            Faction::new(&Race::Raceless, &Color::Colorless),
        ];
        let map = map::open_map(map::MapId::Debug);
        let placer = BuildingPlacer::new(factions, map);

        // Find the row on debug map where (almost) all hexes are native for the color to test
        let native_row_debug_map: usize = match color {
            Color::Yellow => 1,
            Color::Brown => 2,
            Color::Black => 3,
            // 4th row is only water
            Color::Blue => 5,
            Color::Green => 6,
            Color::Gray => 7,
            Color::Red => 8,
            _ => panic!("Invalid color"),
        };

        // Verify that building is only allowed to be placed on the native row
        for row in 0..map::MAP_HEIGHT {
            let actual_valid = placer.valid_placement(player_id, (row, 0));
            let expected_valid = row == native_row_debug_map;
            assert_eq!(actual_valid, expected_valid);
        }
    }

    #[parameterized(num_players = { 2, 3, 4, 5 })]
    fn building_placement_cant_be_finished_prematurely(num_players: usize) {
        let mut placer = create_test_building_placer(num_players);

        placer.place(0, (1, 0)).unwrap();

        assert!(placer.finish().is_err());
    }

    #[parameterized(num_players = { 2, 3, 4, 5 })]
    fn building_cant_be_placed_outside_border(num_players: usize) {
        let mut placer = create_test_building_placer(num_players);
        let player_id = 0;
        let outside_pos_1 = (map::MAP_HEIGHT, 1usize);
        let outside_pos_2 = (1usize, map::MAP_WIDTH);

        assert!(!placer.valid_placement(player_id, outside_pos_1));
        assert!(!placer.valid_placement(player_id, outside_pos_2));
        assert!(placer.place(player_id, outside_pos_1).is_err());
        assert!(placer.place(player_id, outside_pos_2).is_err());
    }

    #[parameterized(num_players = { 2, 3, 4, 5 })]
    fn cant_place_building_on_already_occupied_hex(num_players: usize) {
        let mut placer = create_test_building_placer(num_players);
        let pos = (1, 0);

        placer.place(0, pos).unwrap();

        assert!(placer.place(1, pos).is_err());
    }
}
