use std::fmt::{Debug, Display};
use std::num::{NonZeroU8};

pub trait Player: PartialEq + Copy + Debug + Display + Send + Sync {
    fn other(&self) -> Self;
}

impl Player for NonZeroU8 {
    fn other(&self) -> Self {
        if self.get() == 1 {
            NonZeroU8::new(2).unwrap()
        } else {
            NonZeroU8::new(1).unwrap()
        }
    }
}