use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

pub trait Scalar: Sized + Add + Sub + PartialEq + Ord + Copy + Clone + Display + Debug {
    fn MIN() -> Self;
    fn MAX() -> Self;
}

impl Scalar for i16 {
    fn MIN() -> Self {
        i16::MIN
    }

    fn MAX() -> Self {
        i16::MAX
    }
}

impl Scalar for i32 {
    fn MIN() -> Self {
        i32::MIN
    }

    fn MAX() -> Self {
        i32::MAX
    }
}