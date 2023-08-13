use std::ops;

use enum_iterator::Sequence;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VP(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coins(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tools(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scholars(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Books(pub u32);

pub trait Resource: Sized + Copy + Clone {
    fn from(val: u32) -> Self;
    fn get() -> u32;
}

impl<T: Resource> Resource for T {
    fn from(val: u32) -> Self {
        todo!()
    }

    fn get() -> u32 {
        todo!()
    }
} 

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Discipline {
    Banking,
    Law,
    Engineering,
    Medicine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Sequence)]
pub enum Color {
    Yellow,
    Brown,
    Black,
    Blue,
    Green,
    Gray,
    Red,
}