use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

#[allow(non_snake_case)]
pub trait Scalar: Sized + Add + Sub + PartialEq + Ord + Copy + Clone + Display + Debug {
    fn MIN() -> Self;
    fn MAX() -> Self;

    fn ZERO() -> Self;

    fn div(&self, by: i32) -> Self;
}

#[allow(non_snake_case)]
impl Scalar for i16 {
    fn MIN() -> Self {
        i16::MIN
    }

    fn MAX() -> Self {
        i16::MAX
    }

    fn ZERO() -> Self {
        0
    }

    fn div(&self, by: i32) -> Self {
        (*self as i32 / by) as i16
    }
}

#[allow(non_snake_case)]
impl Scalar for i32 {
    fn MIN() -> Self {
        i32::MIN
    }

    fn MAX() -> Self {
        i32::MAX
    }

    fn ZERO() -> Self {
        0
    }

    fn div(&self, by: i32) -> Self {
        *self / by
    }
}