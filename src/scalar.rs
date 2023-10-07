use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

#[allow(non_snake_case)]
pub trait Scalar:
    Sized + Add + Sub + PartialEq + Ord + Copy + Clone + Display + Debug + Send + Sync + From<i32>
{
    fn MIN() -> Self;
    fn MAX() -> Self;

    fn ZERO() -> Self;

    fn div(&self, by: i32) -> Self;

    /// Adds `add` to `self` if `self` is positive, subtracts `add` from `self` if `self` is negative, else returns `self`
    /// This should not overflow, but instead saturate to `MIN` or `MAX`
    fn add_towards_0(&self, add: i32) -> Self;
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

    fn add_towards_0(&self, add: i32) -> Self {
        if *self > 0 {
            self.saturating_sub(add)
        } else if *self < 0 {
            self.saturating_add(add)
        } else {
            *self
        }
    }
}
