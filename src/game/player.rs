use std::num::{NonZeroU8};

pub trait Player: PartialEq + Copy {
    fn complementary(&self) -> Self;
}

impl Player for NonZeroU8 {
    fn complementary(&self) -> Self {
        if self.get() == 1 {
            NonZeroU8::new(2).unwrap()
        } else {
            NonZeroU8::new(1).unwrap()
        }
    }
}