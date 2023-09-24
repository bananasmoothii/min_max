use std::num::NonZeroU8;
use crate::power4::Power4;
use strum_macros::EnumIter;
use crate::game::Game;

#[derive(EnumIter, Debug)]
pub enum P4IteratorType {
    Vertical,
    Horizontal,
    DiagonalDown,
    DiagonalUp,
}

pub struct BoardIterator<'a> {
    game: &'a Power4,
    pub iterator_type: P4IteratorType,
    pub x: isize,
    pub y: isize,
}

impl BoardIterator<'_> {
    pub fn new_at(game: &Power4, iterator_type: P4IteratorType, x: isize, y: isize) -> BoardIterator {
        BoardIterator {
            game,
            iterator_type,
            x,
            y,
        }
    }

    pub fn get_coords_with_offset(&self, offset: isize) -> (isize, isize) {
        match self.iterator_type {
            P4IteratorType::Horizontal => (self.y, self.x + offset),
            P4IteratorType::Vertical => (self.y + offset, self.x),
            P4IteratorType::DiagonalDown => (self.y + offset, self.x + offset),
            P4IteratorType::DiagonalUp => (self.y - offset, self.x + offset),
        }
    }

    pub fn get_with_offset(&self, offset: isize) -> Option<<Self as Iterator>::Item> {
        let (y, x) = self.get_coords_with_offset(offset);
        if x < 0 || x >= 7 || y < 0 || y >= 6 {
            return None;
        }
        Some(self.game.get_isize((y, x)))
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = Option<<Power4 as Game>::Player>;

    fn next(&mut self) -> Option<Self::Item> {
        let (y, x) = self.get_coords_with_offset(1);
        self.y = y;
        self.x = x;
        if self.x < 0 || self.x >= 7 || self.y < 0 || self.y >= 6 {
            return None;
        }
        Some(self.game.get_isize((y, x)))
    }
}

impl<'a> IntoIterator for &'a Power4 {
    type Item = <BoardIterator<'a> as Iterator>::Item;

    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            game: self,
            iterator_type: P4IteratorType::Horizontal,
            x: 0,
            y: 0,
        }
    }
}
