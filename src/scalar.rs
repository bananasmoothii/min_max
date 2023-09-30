use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

pub trait Scalar: Sized + Add + Sub + PartialEq + Ord + Copy + Clone + Display + Debug {
    fn MIN() -> Self;
    fn MAX() -> Self;

    fn div(&self, by: i32) -> Self;
}

impl Scalar for i16 {
    fn MIN() -> Self {
        i16::MIN
    }

    fn MAX() -> Self {
        i16::MAX
    }

    fn div(&self, by: i32) -> Self {
        (*self as i32 / by) as i16
    }
}

impl Scalar for i32 {
    fn MIN() -> Self {
        i32::MIN
    }

    fn MAX() -> Self {
        i32::MAX
    }

    fn div(&self, by: i32) -> Self {
        *self / by
    }
}