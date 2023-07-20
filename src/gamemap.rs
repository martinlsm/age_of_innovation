use std::{error::Error, fs};

use crate::{error, Result};

pub const MAP_HEIGHT: usize = 9;
pub const MAP_WIDTH: usize = 13;

pub type HexGrid = Vec<Vec<Hex>>;

pub struct Hex {
    pub name: Option<String>,
    pub terrain: TerrainType,
}

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

fn terrain_letter(terrain: &TerrainType) -> char {
    match terrain {
        TerrainType::WATER => 'I',
        TerrainType::YELLOW => 'Y',
        TerrainType::BROWN => 'U',
        TerrainType::BLACK => 'K',
        TerrainType::BLUE => 'B',
        TerrainType::GREEN => 'G',
        TerrainType::GRAY => 'S',
        TerrainType::RED => 'R',
    }
}

fn open_map_from_file(path: &str) -> Result<Vec<Vec<Hex>>> {
    let input = fs::read_to_string(path)?;

    let mut row_name_gen = "ABCDEFGHIJ".chars();
    let mut res: Vec<Vec<Hex>> = Vec::with_capacity(MAP_HEIGHT);

    for row in input.trim().split(';') {
        let row_name = match row_name_gen.next() {
            Some(x) => x,
            None => return Err(error::create_error(&format!("Too many rows"))),
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

/*
    for row in input.split(";") {
        let mut hexes: Vec<Hex> = Vec::with_capacity(MAP_WIDTH);

        let mut hex_col_names = 1..14;
        let next_row_name = hex_row_names.next().unwrap(); // XXX

        for terrain in row.split(",") {
            let terrain = terrain.trim();

            // Hexes should be notated as a single letter
            if terrain.len() != 1 {
                return Err(error::create_error(&format!(
                    "Too many symbols between ',' delimiter (expects 1, got {})",
                    terrain.len()
                )));
            }

            let terrain = terrain.chars().next().unwrap();

            // Water hexes must be specially handled since they have no name.
            if terrain == terrain_letter(&TerrainType::WATER) {
                hexes.push(Hex {
                    name : None,
                    terrain : TerrainType::WATER,
                });

                continue;
            }

            // It is a non-water hex; figure out its name
            let next_col_name = match hex_col_names.next() {
                Some(x) => x,
                None => return Err(error::create_error("Too many hexes on a single row")),
            };

            match terrain {
                'I' => hexes.push(Hex {
                    name: String::from(next_row_name) + &next_col_name.to_string(), // XXX: Do not proceed naming here
                    terrain: TerrainType::WATER,
                }),
                _ => {
                    return Err(error::create_error(&format!(
                        "Unexpected symbol '{}'",
                        terrain
                    )))
                }
            }
        }

        res.push(hexes);
    }

    Err(error::create_error("msg"))
}
*/

/*
fn open_map_from_file(path: &str) -> Result<Vec<Vec<Hex>>> {
    let input = fs::read_to_string(path)?;
    let mut expect_comma = false;
    let mut expect_semilicon_counter = MAP_WIDTH;
    let mut res: Vec<Vec<Hex>> = Vec::new();

    for c in input.chars() {
        if c == ',' {  // Column delimiter
            if expect_comma {
                expect_comma = false;
            } else {
                return Err(error::create_error(&format!("Expected ',' but got '{}'", c)));
            }
        } else if c == ';' {  // Row delimiter
            if expect_semilicon_counter == 0 {
                expect_semilicon_counter = MAP_WIDTH;
            } else {
                return Err(error::create_error(&format!("Expected ';' but got '{}'", c)));
            }
        } else if "IYUKBGSR".contains(c) {
            match c {
            }

        } else {
            return Err(error::create_error(&format!("Got unexpected symbol '{}'", c)));
        }
    }

    let mut grid = Vec::with_capacity(MAP_HEIGHT);
    for _ in 0..MAP_HEIGHT {
        let mut row = Vec::with_capacity(MAP_WIDTH);
        for _ in 0..MAP_WIDTH {
            row.push(Hex {
                name: String::from("A1"),
                terrain: TerrainType::WATER,
            });
        }
        grid.push(row);
    }

    Ok(grid)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_basemap() -> Result<()> {
        open_map_from_file("assets/base_map.gamemap")?;

        Ok(())
    }
}
