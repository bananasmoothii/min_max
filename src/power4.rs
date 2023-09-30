mod iteration;
mod tests;

use std::cmp::min;
use std::num::NonZeroU8;

use crate::game::Game;
use crate::power4::iteration::{BoardIterator, P4IteratorType};

#[derive(Debug, Clone)]
pub struct Power4 {
    board: [[Option<NonZeroU8>; 7]; 6],
    plays: u16,
    last_played_coords: Option<(usize, usize)>,
}

impl Power4 {
    pub fn new() -> Power4 {
        Power4 {
            board: [[None; 7]; 6],
            plays: 0,
            last_played_coords: None,
        }
    }

    /**
     * Returns all iterators for all lines having 4 or more cells
     */
    fn all_lines_longer_4(&self) -> Vec<BoardIterator> {
        let mut iterators: Vec<BoardIterator> = Vec::with_capacity(7 + 6 + 2 * 7); // 7 horizontal + 6 vertical + 2 * 7 diagonal

        for y in 0..6 {
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::Horizontal, 0, y));
        }

        for x in 0..7 {
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::Vertical, x, 0));
        }

        // Diagonal down -> starting here:
        // X  X  X  X  -  -  -
        // X  -  -  -  -  -  -
        // X  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        for x in 0..=3 {
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::DiagonalDown, x, 0));
        }
        for y in 1..=3 {
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::DiagonalDown, 0, y));
        }

        // Diagonal up -> starting here:
        // -  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        // X  -  -  -  -  -  -
        // X  -  -  -  -  -  -
        // X  X  X  X  -  -  -
        for y in 3..=5 {
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::DiagonalUp, 0, y));
        }
        for x in 1..=3 {
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::DiagonalUp, x, 5));
        }

        iterators
    }

    fn lines_passing_at_longer_4(&self, x: usize, y: usize) -> Vec<BoardIterator> {
        let x = x as isize;
        let y = y as isize;
        let mut iterators = vec![
            BoardIterator::new_at(&self, P4IteratorType::Horizontal, 0, y),
            BoardIterator::new_at(&self, P4IteratorType::Vertical, x, 0),
        ];
        if x <= 3 && y <= 3 {
            let subtract = min(x, y);
            // one of x - subtract or y - subtract is 0
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::DiagonalDown, x - subtract, y - subtract));
        }
        let y_from_bottom = 5 - y;
        if x <= 3 && y_from_bottom <= 3 {
            let subtract = min(x, y_from_bottom);
            iterators.push(BoardIterator::new_at(&self, P4IteratorType::DiagonalUp, x - subtract, y + subtract));
        }
        iterators
    }

    fn calculate_score(&self, aligns2: u16, aligns3: u16) -> i32 {
        (5 * aligns2 + 25 * aligns3) as i32
    }

    pub fn get_isize(&self, (row, column): (isize, isize)) -> Option<NonZeroU8> {
        if row < 0 || row >= 6 || column < 0 || column >= 7 {
            return None;
        }
        self.board[row as usize][column as usize]
    }
}

impl Game for Power4 {
    type Coordinate = (usize, usize);

    type InputCoordinate = usize;

    /**
     * The player is represented by 1 or 2
     */
    type Player = NonZeroU8;

    type Score = i32;

    fn get(&self, (row, column): (usize, usize)) -> Option<&NonZeroU8> {
        if row >= 6 || column >= 7 {
            return None;
        }
        self.board[row][column].as_ref()
    }

    fn play<'a>(&mut self, player: NonZeroU8, column: usize) -> Result<(), &'a str> {
        if column >= 7 {
            return Err("Column out of bounds");
        }
        for i in 0..6 {
            if self.board[5 - i][column].is_none() {
                self.board[5 - i][column] = Some(player);
                self.plays += 1;
                return Ok(());
            }
        }
        Err("Column full")
    }

    /**
     * Returns the score of the player, higher is better
     *
     * Scores:
     * - 2 aligned: 5n (n = number of 2 aligned)
     * - 3 aligned: 10^n (n = number of 3 aligned)
     * - 4 aligned: infinite
     * Subtract the same score for the opponent
     */
    fn get_score(&self, player: Self::Player) -> Self::Score {
        let mut p1_aligns2: u16 = 0;
        let mut p1_aligns3: u16 = 0;
        let mut p2_aligns2: u16 = 0;
        let mut p2_aligns3: u16 = 0;
        // let debug_cell = |cell: Option<Option<NonZeroU8>>| cell.map(|c| c.map(|c| c.to_string()).unwrap_or("-".to_string())).unwrap_or("X".to_string());
        let is_playable = |cell: Option<Option<NonZeroU8>>| cell.is_some() && cell.unwrap().is_none();
        for mut line_iterator in self.all_lines_longer_4() {
            let mut strike_player = NonZeroU8::new(1u8).unwrap();
            let mut strike: u8 = 0;
            let mut cell_option = line_iterator.get_with_offset(0);
            while let Some(cell) = cell_option {
                if let Some(cell_player) = cell {
                    if strike_player == cell_player {
                        let before4 = || line_iterator.get_with_offset(-4);
                        let before3 = || line_iterator.get_with_offset(-3);
                        let before2 = || line_iterator.get_with_offset(-2);
                        // let before1 = || line_iterator.get_with_offset(-1);
                        let after1 = || line_iterator.get_with_offset(1);
                        let after2 = || line_iterator.get_with_offset(2);

                        strike += 1;
                        /*
                        println!(
                            "{:?} ({strike}) {} {} {} {} [{}] {} {}",
                            line_iterator.iterator_type,
                            debug_cell(before4()),
                            debug_cell(before3()),
                            debug_cell(before2()),
                            debug_cell(before1()),
                            debug_cell(cell_option),
                            debug_cell(after1()),
                            debug_cell(after2())
                        );
                        */

                        match strike {
                            2 => {
                                if (is_playable(before3()) && is_playable(before2())) // space 2 before
                                    || (is_playable(after1()) && is_playable(after2())) // space 2 after
                                {
                                    if strike_player == player {
                                        p1_aligns2 += 1;
                                    } else {
                                        p2_aligns2 += 1;
                                    }
                                }
                            }
                            3 => {
                                if (is_playable(before4())) // space 1 before
                                    || (is_playable(after1())) // space 1 after
                                {
                                    if strike_player == player {
                                        p1_aligns3 += 1;
                                    } else {
                                        p2_aligns3 += 1;
                                    }
                                }
                            }
                            4 => {
                                return if strike_player == player {
                                    i32::MAX
                                } else {
                                    i32::MIN
                                };
                            }
                            _ => {}
                        }
                    } else {
                        strike_player = cell_player;

                        strike = 1;
                    }
                } else {
                    strike = 0;
                }
                cell_option = line_iterator.next();
            }
        }
        let p1_score = self.calculate_score(p1_aligns2, p1_aligns3) - self.calculate_score(p2_aligns2, p2_aligns3);
        if player == NonZeroU8::new(1u8).unwrap() {
            p1_score
        } else {
            -p1_score
        }
    }

    fn get_winner(&self) -> Option<Self::Player> {
        if self.last_played_coords.is_none() {
            return None;
        }
        let (last_x, last_y) = self.last_played_coords.unwrap();
        for mut line_iterator in self.lines_passing_at_longer_4(last_x, last_y) {
            let mut strike_player = NonZeroU8::new(1u8).unwrap();
            let mut strike: u8 = 0;
            let mut cell_option = line_iterator.get_with_offset(0);
            while let Some(cell) = cell_option {
                if let Some(cell_player) = cell {
                    if strike_player == cell_player {
                        strike += 1;

                        if strike == 4 {
                            return Some(cell_player);
                        }
                    } else {
                        strike_player = cell_player;

                        strike = 1;
                    }
                } else {
                    strike = 0;
                }
                cell_option = line_iterator.next();
            }
        }
        None
    }


    fn is_full(&self) -> bool {
        for i in 0..6 {
            if self.board[0][i].is_none() {
                return false;
            }
        }
        true
    }

    fn possible_plays(&self) -> Vec<usize> {
        (0..=6)
            .filter(|&column| self.get((0, column)).is_none())
            .collect::<Vec<usize>>()
    }

    fn print(&self) {
        println!("1 2 3 4 5 6 7");
        for i in 0..6 {
            for j in 0..7 {
                print!("{} ", self.board[i][j]
                    .map(|cell| cell.to_string())
                    .unwrap_or("-".to_string()));
            }
            println!();
        }
    }

    fn plays(&self) -> u16 {
        self.plays
    }

    fn last_play(&self) -> Option<Self::InputCoordinate> {
        self.last_played_coords.map(|(x, _)| x)
    }
}
