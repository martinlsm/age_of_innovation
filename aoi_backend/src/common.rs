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
