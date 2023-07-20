use std::fs;

use crate::{error, Result};

pub const MAP_HEIGHT: usize = 9;
pub const MAP_WIDTH: usize = 13;

pub type HexGrid = Vec<Vec<Hex>>;

pub struct Hex {
    pub name: Option<String>,
    pub terrain: TerrainType,
}

#[derive(PartialEq)]
pub enum TerrainType {
    WATER,
    YELLOW,
    BROWN,
    BLACK,
    BLUE,
    GREEN,
    GRAY,
    RED,
}

fn open_map_from_file(path: &str) -> Result<Vec<Vec<Hex>>> {
    let input = fs::read_to_string(path)?;

    let mut row_name_gen = "ABCDEFGHIJ".chars();
    let mut res: Vec<Vec<Hex>> = Vec::with_capacity(MAP_HEIGHT);

    for row in input.split('\n') {
        let row_name = match row_name_gen.next() {
            Some(x) => x,
            None => return Err(error::create_error("Too many rows")),
        };
        let hexes: Vec<Hex> = parse_row(&row, row_name)?;
        res.push(hexes);
    }

    Ok(res)
}

fn parse_row(input: &str, row_name: char) -> Result<Vec<Hex>> {
    let mut hex_name_gen = (1..).map(|n| format!("{}{}", row_name, n));

    let row: Result<Vec<Hex>> = input
        .split(",")
        .inspect(|s| println!("'{}'", s))
        .map(|x| {
            match x {
                "I" => Ok(TerrainType::WATER),
                "Y" => Ok(TerrainType::YELLOW),
                "U" => Ok(TerrainType::BROWN),
                "K" => Ok(TerrainType::BLACK),
                "B" => Ok(TerrainType::BLUE),
                "G" => Ok(TerrainType::GREEN),
                "S" => Ok(TerrainType::GRAY),
                "R" => Ok(TerrainType::RED),
                x => return Err(error::create_error(&format!("Invalid symbol '{}'", x))),
            }
            .and_then(|t| match t {
                TerrainType::WATER => Ok(Hex {
                    name: None,
                    terrain: t,
                }),
                _ => Ok(Hex {
                    name: hex_name_gen.next(),
                    terrain: t,
                }),
            })
        })
        .collect();

    row.and_then(|v| match v.len() {
        MAP_WIDTH => Ok(v),
        _ => Err(error::create_error(&format!(
            "Incorrect width of map at row '{}' (expects {})",
            row_name, MAP_WIDTH
        ))),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn import_basemap() -> Result<()> {
        let map = open_map_from_file("assets/base_map.gamemap")?;

        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::WATER)
                .count(),
            36
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::YELLOW)
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::BROWN)
                .count(),
            12
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::BLACK)
                .count(),
            12
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::BLUE)
                .count(),
            12
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::GREEN)
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::GRAY)
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::GRAY)
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == TerrainType::RED)
                .count(),
            12
        );

        assert!(map.len() == MAP_HEIGHT);
        assert!(map.into_iter().all(|row| row.len() == MAP_WIDTH));

        Ok(())
    }
}
