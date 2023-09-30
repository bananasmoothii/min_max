use std::fmt::{Display, Formatter};
use crate::game::Game;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameState<G: Game> {
    PlayersTurn(G::Player),
    Draw,
    WonBy(G::Player),
}

impl<G: Game> Display for GameState<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::PlayersTurn(player) => write!(f, "Player {}'s turn", player),
            GameState::Draw => write!(f, "Draw"),
            GameState::WonBy(player) => write!(f, "Player {} won", player),
        }
    }
}
