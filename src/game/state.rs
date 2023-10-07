use std::fmt::{Display, Formatter};

use crate::game::player::Player;
use crate::game::Game;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState<G: Game> {
    PlayersTurn(G::Player, Option<G::InputCoordinate>),
    Draw(G::Player, G::InputCoordinate),
    WonBy(G::Player, G::InputCoordinate),
}

impl<G: Game> Display for GameState<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::PlayersTurn(player, input) => write!(
                f,
                "Player {player}'s turn (last play: {})",
                input.map(|p| p.to_string()).unwrap_or("-".to_string())
            ),
            GameState::Draw(last_player, input) => {
                write!(f, "Draw (last play: {input} by {last_player})")
            }
            GameState::WonBy(player, input) => {
                write!(f, "Player {player} won (last play: {input})")
            }
        }
    }
}

impl<G: Game> GameState<G> {
    pub fn get_last_play(&self) -> (G::Player, Option<G::InputCoordinate>) {
        match self {
            GameState::PlayersTurn(player, input) => (player.other(), *input),
            GameState::Draw(last_player, input) => (*last_player, Some(*input)),
            GameState::WonBy(player, input) => (*player, Some(*input)),
        }
    }

    pub fn to_draw(&self) -> GameState<G> {
        match self {
            GameState::PlayersTurn(current_player, last_input) => GameState::Draw(
                current_player.other(),
                last_input.expect("Cannot draw when no play has been made"),
            ),
            _ => panic!("game is not at playing state, but at {}", self),
        }
    }

    pub fn to_win(&self) -> GameState<G> {
        match self {
            GameState::PlayersTurn(current_player, last_input) => GameState::WonBy(
                current_player.other(),
                last_input.expect("Cannot win when no play has been made"),
            ),
            _ => panic!("game is not at playing state, but at {}", self),
        }
    }

    pub fn to_win_by(&self, winner: G::Player) -> GameState<G> {
        match self {
            GameState::PlayersTurn(_, last_input) => GameState::WonBy(
                winner,
                last_input.expect("Cannot win when no play has been made"),
            ),
            _ => panic!("game is not at playing state, but at {}", self),
        }
    }
}
