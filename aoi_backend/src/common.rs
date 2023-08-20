use std::{ops, iter};

use enum_iterator::Sequence;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VP(pub u32);

pub trait Resource:
    From<u32> + Copy + Clone + ops::Add<Output = Self> + ops::Sub<Output = Self>
{
    const IDX: usize;

    fn get_val(&self) -> u32;
}

macro_rules! define_resource {
    ($name:ident, $idx:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub u32);

        impl Resource for $name {
            const IDX: usize = $idx;

            fn get_val(&self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }

        impl From<$name> for u32 {
            fn from(val: $name) -> Self {
                val.0
            }
        }

        impl ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
    };
}

define_resource!(Tools, 0);
define_resource!(Coins, 1);
define_resource!(Scholars, 2);
define_resource!(Books, 3);
define_resource!(Power, 4);
const NUM_RESOURCES: usize = 5;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resources {
    amounts: Vec<u32>, // Resources are stored in their respective Resource::IDX.
}

impl Resources {
    pub fn new() -> Self {
        Self {
            amounts: iter::repeat(0).take(NUM_RESOURCES).collect()
        }
    }

    pub fn get<T: Resource>(&self) -> T {
        T::from(self.amounts[T::IDX])
    }

    pub fn gain<T: Resource>(&mut self, resource: &T) -> T {
        self.amounts[T::IDX] += resource.get_val();

        self.get()
    }

    pub fn lose<T: Resource>(&mut self, resource: &T) -> T {
        self.amounts[T::IDX] -= resource.get_val();

        self.get()
    }
}

impl<T: Resource> From<T> for Resources {
    fn from(value: T) -> Self {
        let mut res = Resources::new();
        res.amounts[T::IDX] = value.get_val();

        res
    }
}

impl ops::Add<&Resources> for Resources {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        // Pointwise addition.
        let amounts: Vec<u32> = std::iter::zip(self.amounts.iter(), rhs.amounts.iter())
            .map(|(a, b)| a + b)
            .collect();

        Self { amounts }
    }
}

impl ops::Sub<&Resources> for Resources {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        // Pointwise subtraction.
        let amounts: Vec<u32> = std::iter::zip(self.amounts.iter(), rhs.amounts.iter())
            .map(|(a, b)| a - b)
            .collect();

        Self { amounts }
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
