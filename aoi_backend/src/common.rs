use enum_iterator::Sequence;

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VP(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Discipline {
    Banking,
    Law,
    Engineering,
    Medicine,
}

pub const DISCIPLINE_MAX: u32 = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Sequence, Serialize)]
pub enum Color {
    Yellow,
    Brown,
    Black,
    Blue,
    Green,
    Gray,
    Red,
    Colorless, // For testing
}
