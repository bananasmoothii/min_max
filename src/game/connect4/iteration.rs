use strum_macros::EnumIter;

use crate::game::connect4::ConnectFour;
use crate::game::Game;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
pub enum P4IteratorType {
    Vertical,
    Horizontal,
    DiagonalDown,
    DiagonalUp,
}

pub struct BoardIterator<'a> {
    game: &'a ConnectFour,
    pub iterator_type: P4IteratorType,
    pub y: isize,
    pub x: isize,
}

impl BoardIterator<'_> {
    pub fn new_at(
        game: &ConnectFour,
        iterator_type: P4IteratorType,
        x: isize,
        y: isize,
    ) -> BoardIterator {
        BoardIterator {
            game,
            iterator_type,
            y,
            x,
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

    pub fn increment(&mut self) -> (isize, isize) {
        let (y, x) = self.get_coords_with_offset(1);
        self.y = y;
        self.x = x;
        (y, x)
    }

    pub fn get_with_offset(&self, offset: isize) -> Option<<Self as Iterator>::Item> {
        let (y, x) = self.get_coords_with_offset(offset);
        if x < 0 || x >= 7 || y < 0 || y >= 6 {
            return None;
        }
        Some(self.game.get_isize((y, x)))
    }

    pub fn to_string(self) -> String {
        let mut result = String::new();
        for cell in self {
            result.push(match cell {
                Some(player) => match player.get() {
                    1 => '1',
                    2 => '2',
                    _ => '-',
                },
                None => 'X',
            });
        }
        result
    }

    pub fn coords(&self) -> (isize, isize) {
        (self.y, self.x)
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = Option<<ConnectFour as Game>::Player>;

    fn next(&mut self) -> Option<Self::Item> {
        let (y, x) = self.increment();
        if self.x < 0 || self.x >= 7 || self.y < 0 || self.y >= 6 {
            return None;
        }
        Some(self.game.get_isize((y, x)))
    }
}

impl<'a> IntoIterator for &'a ConnectFour {
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
