use std::fmt::Display;
use std::hash::Hash;

use crate::game::player::Player;
use crate::scalar::Scalar;

pub mod connect4;
pub mod player;
pub(crate) mod state;

pub trait Game: Clone + Send + Sync {
    type Coordinate;

    type InputCoordinate: Copy + Eq + Ord + Hash + Display + Send + Sync;

    type Player: Player;

    type Score: Scalar;

    fn new() -> Self;

    fn get(&self, coordinate: Self::Coordinate) -> Option<&Self::Player>;

    fn play<'a>(
        &mut self,
        player: Self::Player,
        coordinate: Self::InputCoordinate,
    ) -> Result<(), &'a str>;

    fn get_score(&self, player: Self::Player) -> Self::Score;

    fn get_winner(&self) -> Option<Self::Player>;

    fn is_full(&self) -> bool;

    fn possible_plays(&self) -> Vec<Self::InputCoordinate>;

    fn print(&self);

    /**
     * Number of plays made in the game
     */
    fn plays(&self) -> u16;

    /**
     * Last play made in the game. None only if no play has been made yet.
     */
    fn last_play(&self) -> Option<Self::InputCoordinate>;
}
