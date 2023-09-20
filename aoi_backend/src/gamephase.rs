use crate::{building::Building, common::Color};

pub mod buildingplacement;
pub mod incomephase;

#[derive(Debug)]
pub struct HexState {
    color: Color,
    building: Option<Building>,
}