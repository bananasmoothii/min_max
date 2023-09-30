use crate::game::Game;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameState<G: Game> {
    PlayersTurn(G::Player),
    Draw,
    WonBy(G::Player),
}
