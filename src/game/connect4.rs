use std::cmp::min;
use std::num::NonZeroU8;

use console::Style;

use crate::game::connect4::count_direction::CountDirection;
use crate::game::connect4::iteration::{BoardIterator, P4IteratorType};
use crate::game::Game;

mod count_direction;
mod iteration;
mod tests;

#[derive(Debug, Clone)]
pub struct Power4 {
    board: [[Option<NonZeroU8>; 7]; 6],
    plays: u16,
    last_played_coords: Option<(usize, usize)>,
}

impl Power4 {
    /**
     * Returns all iterators for all lines having 4 or more cells
     */
    fn all_lines_longer_4(&self) -> Vec<BoardIterator> {
        let mut iterators: Vec<BoardIterator> = Vec::with_capacity(7 + 6 + 2 * 7); // 7 horizontal + 6 vertical + 2 * 7 diagonal

        for y in 0..6 {
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::Horizontal,
                0,
                y,
            ));
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
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::DiagonalDown,
                x,
                0,
            ));
        }
        for y in 1..=3 {
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::DiagonalDown,
                0,
                y,
            ));
        }

        // Diagonal up -> starting here:
        // -  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        // -  -  -  -  -  -  -
        // X  -  -  -  -  -  -
        // X  -  -  -  -  -  -
        // X  X  X  X  -  -  -
        for y in 3..=5 {
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::DiagonalUp,
                0,
                y,
            ));
        }
        for x in 1..=3 {
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::DiagonalUp,
                x,
                5,
            ));
        }

        iterators
    }

    fn lines_passing_at_longer_4(
        &self,
        coords: <Power4 as Game>::Coordinate,
    ) -> Vec<BoardIterator> {
        let y = coords.0 as isize;
        let x = coords.1 as isize;
        let mut iterators = Vec::with_capacity(4);

        iterators.push(BoardIterator::new_at(
            &self,
            P4IteratorType::Horizontal,
            0,
            y,
        ));
        iterators.push(BoardIterator::new_at(&self, P4IteratorType::Vertical, x, 0));

        let x_from_left = 6 - x;
        let y_from_bottom = 5 - y;

        if x + y_from_bottom >= 3 && x_from_left + y >= 3 {
            let subtract = min(x, y);
            // one of x - subtract or y - subtract is 0
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::DiagonalDown,
                x - subtract,
                y - subtract,
            ));
        }

        if x + y >= 3 && x_from_left + y_from_bottom >= 3 {
            let subtract = min(x, y_from_bottom);
            iterators.push(BoardIterator::new_at(
                &self,
                P4IteratorType::DiagonalUp,
                x - subtract,
                y + subtract,
            ));
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

    pub fn get_winner_coords(
        &self,
    ) -> Option<[<Self as Game>::Coordinate; Self::CONNECT as usize]> {
        if self.last_played_coords.is_none() {
            return None;
        }
        if self.plays < 7 {
            // No winner before 7 plays
            return None;
        }
        let last_coords = self.last_played_coords.unwrap();
        let connect = Self::CONNECT as usize;
        for mut line_iterator in self.lines_passing_at_longer_4(last_coords) {
            let mut winner_coords: Vec<(isize, isize)> = Vec::with_capacity(2 * connect - 1);
            let mut strike_player = NonZeroU8::new(1u8).unwrap();
            let mut strike: u8 = 0;
            let mut cell_option = line_iterator.get_with_offset(0);
            while let Some(cell) = cell_option {
                winner_coords.push(line_iterator.coords());
                if let Some(cell_player) = cell {
                    if strike_player == cell_player {
                        strike += 1;

                        if strike == Self::CONNECT {
                            let mut result = [(0usize, 0usize); Self::CONNECT as usize];
                            let winner_coords_size = winner_coords.len();
                            for i in 0..connect {
                                let (y, x) = winner_coords[winner_coords_size - connect + i];
                                result[i] = (y as usize, x as usize);
                            }
                            return Some(result);
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

    fn count_in_direction(
        &self,
        start: <Power4 as Game>::Coordinate,
        direction: CountDirection,
        max: u8,
    ) -> u8 {
        let player_to_count = self.get(start);
        if player_to_count.is_none() {
            return 0;
        }
        let mut count: u8 = 0;
        let player_to_count = player_to_count.unwrap();
        let mut coords = direction.add_to(start);
        while let Some(player) = coords.map(|coords| self.get(coords)).flatten() {
            if player == player_to_count {
                count += 1;
                if count == max {
                    return count;
                }
            } else {
                break;
            }
            coords = direction.add_to(coords.unwrap());
        }
        count
    }

    const CONNECT: u8 = 4; // should be 4 for connect-4
}

impl Game for Power4 {
    /// (row, column) or (y, x)
    type Coordinate = (usize, usize);

    type InputCoordinate = usize;

    /**
     * The player is represented by 1 or 2
     */
    type Player = NonZeroU8;

    type Score = i32;

    fn new() -> Power4 {
        Power4 {
            board: [[None; 7]; 6],
            plays: 0,
            last_played_coords: None,
        }
    }

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
            let y = 5 - i;
            if self.board[y][column].is_none() {
                self.board[y][column] = Some(player);
                self.plays += 1;
                self.last_played_coords = Some((y, column));
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
        let is_playable =
            |cell: Option<Option<NonZeroU8>>| cell.is_some() && cell.unwrap().is_none();
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
                            Self::CONNECT => {
                                return if strike_player == player {
                                    i32::MAX
                                } else {
                                    i32::MIN
                                };
                            }
                            2 => {
                                if (is_playable(before3()) && is_playable(before2())) // space 2 before
                                    || (is_playable(after1()) && is_playable(after2()))
                                // space 2 after
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
                                    || (is_playable(after1()))
                                // space 1 after
                                {
                                    if strike_player == player {
                                        p1_aligns3 += 1;
                                    } else {
                                        p2_aligns3 += 1;
                                    }
                                }
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
        let p1_score = self.calculate_score(p1_aligns2, p1_aligns3)
            - self.calculate_score(p2_aligns2, p2_aligns3);
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
        let last_coords = self.last_played_coords.unwrap();
        let counting_player = *self.get(last_coords).unwrap();
        let connect_minus1 = Self::CONNECT - 1;
        for count_direction in CountDirection::half_side() {
            // max is 3 because we don't count the middle/start cell
            let count = self.count_in_direction(last_coords, count_direction, connect_minus1);
            if count == connect_minus1 {
                return Some(counting_player);
            }
            let count_opposite = self.count_in_direction(
                last_coords,
                count_direction.opposite(),
                connect_minus1 - count,
            );
            if count + count_opposite == connect_minus1 {
                return Some(counting_player);
            }
        }
        None
    }

    fn is_full(&self) -> bool {
        for i in 0..7 {
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
        let p1_color = Style::new().red();
        let p2_color = Style::new().blue();
        let no_player_color = Style::new().white();

        let win_coords = self.get_winner_coords();

        let bg_if_win = |style: Style, y: usize, x: usize| -> Style {
            if let Some(coords) = &win_coords {
                if coords.contains(&(y, x)) {
                    return style.on_yellow();
                }
            }
            style
        };

        println!("1 2 3 4 5 6 7");
        for y in 0..6 {
            for x in 0..7 {
                let cell_str = self.board[y][x]
                    .map(|cell| cell.to_string())
                    .unwrap_or("-".to_string());
                let style = bg_if_win(
                    match cell_str.as_str() {
                        "1" => p1_color.clone(),
                        "2" => p2_color.clone(),
                        _ => no_player_color.clone(),
                    },
                    y,
                    x,
                );
                print!("{} ", style.apply_to(cell_str));
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
