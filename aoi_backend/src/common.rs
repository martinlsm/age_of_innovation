use std::ops;

use enum_iterator::Sequence;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VP(pub u32);

pub trait Resource: Sized + Copy + Clone + ops::Add<Output = Self> {
    fn from(val: u32) -> Self;
}

macro_rules! define_resource {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub u32);

        impl Resource for $name {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }

        impl ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
    };
}

define_resource!(Tools);
define_resource!(Coins);
define_resource!(Scholars);
define_resource!(Books);

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
