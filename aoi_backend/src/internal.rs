use std::ops;


pub trait Resource: Sized + Copy + Clone + ops::Add<Self, Output = Self> {
    fn from(val: u32) -> Self;
}

impl<T: Resource> ops::Add<u32> for T {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        rhs
    }
}
