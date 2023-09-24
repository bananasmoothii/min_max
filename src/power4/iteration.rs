use crate::power4::Power4;
use strum_macros::EnumIter;
use crate::game::Game;

#[derive(EnumIter)]
pub enum P4IteratorType {
    Vertical,
    Horizontal,
    DiagonalDown,
    DiagonalUp,
}

pub struct BoardIterator<'a> {
    game: &'a Power4,
    pub iterator_type: P4IteratorType,
    pub x: usize,
    pub y: usize,
}

impl BoardIterator<'_> {
    pub fn new_at(game: &Power4, iterator_type: P4IteratorType, x: usize, y: usize) -> BoardIterator {
        BoardIterator {
            game,
            iterator_type,
            x,
            y,
        }
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = Option<&'a <Power4 as Game>::Player>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator_type {
            P4IteratorType::Horizontal => {
                if self.x >= 7 {
                    return None;
                }
                let result = self.game.get((self.y, self.x));
                self.x += 1;
                Some(result)
            }
            P4IteratorType::Vertical => {
                if self.y >= 6 {
                    return None;
                }
                let result = self.game.get((self.y, self.x));
                self.y += 1;
                Some(result)
            }
            P4IteratorType::DiagonalDown => {
                if self.x >= 7 || self.y >= 6 {
                    return None;
                }
                let result = self.game.get((self.y, self.x));
                self.x += 1;
                self.y += 1;
                Some(result)
            }
            P4IteratorType::DiagonalUp => {
                if self.x >= 7 { // usize is always >= 0
                    return None;
                }
                let result = self.game.get((self.y, self.x));
                self.x += 1;
                if self.y == 0 {
                    self.y = usize::MAX;
                } else {
                    self.y -= 1;
                }
                Some(result)
            }
        }
    }
}

impl<'a> IntoIterator for &'a Power4 {
    type Item = Option<&'a <Power4 as Game>::Player>;

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
