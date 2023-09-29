pub mod player;

use crate::game::player::Player;
use crate::scalar::Scalar;

pub trait Game : Clone {
    type Coordinate;

    type InputCoordinate: Copy;

    type Player: Player;

    type Score: Scalar;

    fn get(&self, coordinate: Self::Coordinate) -> Option<&Self::Player>;

    fn play<'a>(&mut self, player: Self::Player, coordinate: Self::InputCoordinate) -> Result<(), &'a str>;

    fn get_score(&self, player: Self::Player) -> Self::Score;

    fn get_winner(&self) -> Option<Self::Player>;

    fn is_full(&self) -> bool;

    fn possible_plays(&self) -> Vec<Self::InputCoordinate>;

    fn print(&self);

    /**
     * Number of plays made in the game
     */
    fn plays(&self) -> u16;
}