use std::cmp::min;
use std::num::{NonZeroU8, NonZeroUsize};

use console::Style;
use rand::Rng;

use crate::game::connect4::count_direction::CountDirection;
use crate::game::connect4::iteration::{BoardIterator, P4IteratorType};
use crate::game::Game;

mod count_direction;
mod iteration;
mod tests;

#[derive(Debug, Clone)]
pub struct ConnectFour {
    board: [[Option<NonZeroU8>; 7]; 6],
    last_played_coords: Option<(usize, usize)>,
    winner: Option<NonZeroU8>,
    p1_aligns2: u16,
    p1_aligns3: u16,
    p2_aligns2: u16,
    p2_aligns3: u16,
}

impl ConnectFour {
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
        coords: <ConnectFour as Game>::Coordinate,
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
        (10 * aligns2 + 100 * aligns3) as i32
    }

    pub fn get_isize(&self, (row, column): (isize, isize)) -> Option<NonZeroU8> {
        if row < 0 || row >= 6 || column < 0 || column >= 7 {
            return None;
        }
        self.board[row as usize][column as usize]
    }

    pub fn play_usize(&mut self, player: NonZeroU8, column: usize) -> Result<(), &str> {
        self.play(player, NonZeroUsize::new(column).unwrap())
    }

    pub fn get_winner_coords(&self) -> Option<[<Self as Game>::Coordinate; 4]> {
        if self.last_played_coords.is_none() {
            return None;
        }
        let last_coords = self.last_played_coords.unwrap();
        for mut line_iterator in self.lines_passing_at_longer_4(last_coords) {
            let mut winner_coords: Vec<(isize, isize)> = Vec::with_capacity(2 * 4 - 1);
            let mut strike_player = NonZeroU8::new(1u8).unwrap();
            let mut strike: u8 = 0;
            let mut cell_option = line_iterator.get_with_offset(0);
            while let Some(cell) = cell_option {
                winner_coords.push(line_iterator.coords());
                if let Some(cell_player) = cell {
                    if strike_player == cell_player {
                        strike += 1;

                        if strike == 4 {
                            let mut result = [(0usize, 0usize); 4];
                            let winner_coords_size = winner_coords.len();
                            for i in 0..4 {
                                let (y, x) = winner_coords[winner_coords_size - 4 + i];
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
        start: <ConnectFour as Game>::Coordinate,
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

    fn compute_aligments(&mut self) {
        /*
        let mut p1_aligns2 = 0;
        let mut p1_aligns3 = 0;
        let mut p2_aligns2 = 0;
        let mut p2_aligns3 = 0;

        // let debug_cell = |cell: Option<Option<NonZeroU8>>| cell.map(|c| c.map(|c| c.to_string()).unwrap_or("-".to_string())).unwrap_or("X".to_string());
        let is_playable =
            |cell: Option<Option<NonZeroU8>>| cell.is_some() && cell.unwrap().is_none();
        for mut line_iterator in self.all_lines_longer_4() {
            let mut strike_player = 0u8; // this value is never used
            let mut strike: u8 = 0;
            let mut cell_option = line_iterator.get_with_offset(0);
            while let Some(cell) = cell_option {
                if let Some(cell_player) = cell {
                    let cell_player_u8 = cell_player.get();
                    if strike_player == cell_player_u8 {
                        let before3 = || line_iterator.get_with_offset(-3);
                        let before2 = || line_iterator.get_with_offset(-2);
                        // let before1 = || line_iterator.get_with_offset(-1);
                        let after1 = || line_iterator.get_with_offset(1);
                        let after2 = || line_iterator.get_with_offset(2);

                        strike += 1;

                        match strike {
                            Self::CONNECT => {
                                self.winner = Some(cell_player);
                                return;
                            }
                            2 => {
                                if (is_playable(before3()) && is_playable(before2())) // space 2 before
                                    || (is_playable(after1()) && is_playable(after2()))
                                // space 2 after
                                {
                                    if strike_player == 1u8 {
                                        p1_aligns2 += 1;
                                    } else {
                                        p2_aligns2 += 1;
                                    }
                                }
                            }
                            3 => {
                                if is_playable(before3()) // space 1 before
                                    || is_playable(after1()) // space 1 after
                                {
                                    if strike_player == 1u8 {
                                        p1_aligns3 += 1;
                                    } else {
                                        p2_aligns3 += 1;
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        strike_player = cell_player_u8;

                        strike = 1;
                    }
                } else {
                    strike = 0;
                }
                cell_option = line_iterator.next();
            }
        }
        self.p1_aligns2 = p1_aligns2;
        self.p1_aligns3 = p1_aligns3;
        self.p2_aligns2 = p2_aligns2;
        self.p2_aligns3 = p2_aligns3;

         */
        if self.last_played_coords.is_none() {
            return;
        }
        let last_coords = self.last_played_coords.unwrap();
        let counting_player = *self.get(last_coords).unwrap();
        for count_direction in CountDirection::half_side() {
            // max is 3 (4 - 1) because we don't count the middle/start cell
            let count = self.count_in_direction(last_coords, count_direction, 3);
            if count == 3 {
                self.winner = Some(counting_player);
                return;
            }
            let count_opposite =
                self.count_in_direction(last_coords, count_direction.opposite(), 3 - count);
            if count + count_opposite == 3 {
                self.winner = Some(counting_player);
                return;
            }

            let counting_player_u8 = counting_player.get();

            if count == 2 || count_opposite == 2 {
                // implies that the other count is <= 1
                // total: 3
                if counting_player_u8 == 1u8 {
                    self.p1_aligns2 -= 1;
                    self.p1_aligns3 += 1;
                } else {
                    self.p2_aligns2 -= 1;
                    self.p2_aligns3 += 1;
                }
            } else if count + count_opposite == 1 {
                // total: 2
                if counting_player_u8 == 1u8 {
                    self.p1_aligns2 += 1;
                } else {
                    self.p2_aligns2 += 1;
                }
            }
        }
    }

    const RANDOMIZE_POSSIBLE_PLAYS: bool = true;
}

impl Game for ConnectFour {
    /// (row, column) or (y, x). Starts at (0, 0) at the top left corner and ends at (5, 6) at the
    /// bottom right corner
    type Coordinate = (usize, usize);

    /// an usize from 1 to 7
    type InputCoordinate = NonZeroUsize;

    /**
     * The player is represented by 1 or 2
     */
    type Player = NonZeroU8;

    type Score = i32;

    fn new() -> ConnectFour {
        ConnectFour {
            board: [[None; 7]; 6],
            last_played_coords: None,
            winner: None,
            p1_aligns2: 0,
            p1_aligns3: 0,
            p2_aligns2: 0,
            p2_aligns3: 0,
        }
    }

    fn get(&self, (row, column): (usize, usize)) -> Option<&NonZeroU8> {
        if row >= 6 || column >= 7 {
            return None;
        }
        self.board[row][column].as_ref()
    }

    fn play<'a>(&mut self, player: NonZeroU8, column: NonZeroUsize) -> Result<(), &'a str> {
        let column_min1 = column.get() - 1;
        if column_min1 >= 7 {
            return Err("Column out of bounds");
        }
        for i in 0..6 {
            let y = 5 - i;
            if self.board[y][column_min1].is_none() {
                self.board[y][column_min1] = Some(player);
                self.last_played_coords = Some((y, column_min1));
                self.compute_aligments();
                return Ok(());
            }
        }
        Err("Column full")
    }

    /**
     * Returns the score of the player, higher is better
     *
     * Scores:
     * - 2 aligned: 20n (n = number of 2 aligned)
     * - 3 aligned: 100n (n = number of 3 aligned)
     * - 4 aligned: infinite
     * Subtract the same score for the opponent
     * Scores are invalid if the line cannot be completed
     */
    fn get_score(&self, player: Self::Player) -> Self::Score {
        if let Some(winner) = self.winner {
            return if winner == player {
                Self::Score::MAX
            } else {
                Self::Score::MIN
            };
        }
        let p1_score = self.calculate_score(self.p1_aligns2, self.p1_aligns3)
            - self.calculate_score(self.p2_aligns2, self.p2_aligns3);
        if player.get() == 1u8 {
            p1_score
        } else {
            -p1_score
        }
    }

    fn get_winner(&self) -> Option<Self::Player> {
        /*
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

         */
        self.winner
    }

    fn is_full(&self) -> bool {
        for i in 0..7 {
            if self.board[0][i].is_none() {
                return false;
            }
        }
        true
    }

    fn possible_plays(&self) -> Vec<NonZeroUsize> {
        let order: [usize; 7] = if Self::RANDOMIZE_POSSIBLE_PLAYS {
            match rand::thread_rng().gen_range(0..=2) {
                0 => [4, 3, 5, 2, 6, 1, 7],
                1 => [3, 5, 4, 6, 2, 1, 7],
                _ => [6, 2, 4, 5, 3, 1, 7],
            }
        } else {
            [4, 3, 5, 2, 6, 1, 7]
        };
        let mut vec: Vec<NonZeroUsize> = Vec::with_capacity(7);
        order
            .iter()
            .filter(|&column| self.get((0, column - 1)).is_none())
            .for_each(|&column| vec.push(NonZeroUsize::new(column).unwrap()));
        vec
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

    fn last_play(&self) -> Option<Self::InputCoordinate> {
        self.last_played_coords
            .map(|(x, _)| NonZeroUsize::new(x).unwrap())
    }
}
