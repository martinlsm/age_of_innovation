use enum_iterator::Sequence;

use crate::{error::create_error, Result, common::Color};

pub const MAP_HEIGHT: usize = 9;
pub const MAP_WIDTH: usize = 13;

pub struct Hex {
    pub name: Option<String>,
    pub terrain: Terrain,
}

#[derive(PartialEq, Sequence)]
pub enum Terrain {
    Land(Color),
    Water,
}

pub fn open_map() -> Vec<Vec<Hex>> {
    const BASE_MAP: &str = include_str!("../assets/base_map.gamemap");
    open_map_from_str(BASE_MAP).unwrap()
}

fn open_map_from_str(input: &str) -> Result<Vec<Vec<Hex>>> {
    let mut row_name_gen = "ABCDEFGHIJ".chars();
    let mut res: Vec<Vec<Hex>> = Vec::with_capacity(MAP_HEIGHT);

    for row in input.split('\n') {
        let row_name = match row_name_gen.next() {
            Some(x) => x,
            None => return Err(create_error("Too many rows")),
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
        .map(|x| {
            match x {
                "I" => Ok(Terrain::Water),
                "Y" => Ok(Terrain::Land(Color::Yellow)),
                "U" => Ok(Terrain::Land(Color::Brown)),
                "K" => Ok(Terrain::Land(Color::Black)),
                "B" => Ok(Terrain::Land(Color::Blue)),
                "G" => Ok(Terrain::Land(Color::Green)),
                "S" => Ok(Terrain::Land(Color::Gray)),
                "R" => Ok(Terrain::Land(Color::Red)),
                x => return Err(create_error(&format!("Invalid symbol '{}'", x))),
            }
            .and_then(|t| match t {
                Terrain::Water => Ok(Hex {
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
        _ => Err(create_error(&format!(
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
        let map = open_map();

        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Water)
                .count(),
            36
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Yellow))
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Brown))
                .count(),
            12
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Black))
                .count(),
            12
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Blue))
                .count(),
            12
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Green))
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Gray))
                .count(),
            11
        );
        assert_eq!(
            (0..MAP_HEIGHT)
                .cartesian_product(0..MAP_WIDTH)
                .filter(|&(r, c)| map[r][c].terrain == Terrain::Land(Color::Red))
                .count(),
            12
        );

        assert!(map.len() == MAP_HEIGHT);
        assert!(map.into_iter().all(|row| row.len() == MAP_WIDTH));

        Ok(())
    }
}
