use std::fmt::{Debug, Formatter};
use crate::game::state::GameState;
use crate::game::Game;
use crate::game::state::GameState::PlayersTurn;

pub struct GameNode<G: Game> {
    depth: u32,
    weight: Option<G::Score>,
    children: Vec<GameNode<G>>,
    pub game: G,
    pub game_state: GameState<G>,
}

impl<G: Game> GameNode<G> {
    pub fn new(game: G, depth: u32, weight: Option<G::Score>, game_state: GameState<G>) -> GameNode<G> {
        GameNode {
            depth,
            weight,
            children: Vec::new(),
            game,
            game_state,
        }
    }

    pub fn new_root(game: G, starting_player: G::Player) -> GameNode<G> {
        GameNode::new(game, 0, None, PlayersTurn(starting_player))
    }

    // Getters

    pub fn depth(&self) -> u32 {
        self.depth
    }
    pub fn weight(&self) -> Option<G::Score> {
        self.weight
    }
    pub fn children(&self) -> &Vec<GameNode<G>> {
        &self.children
    }
    pub fn children_mut(&mut self) -> &mut Vec<GameNode<G>> {
        &mut self.children
    }

    // Setters

    pub fn set_weight(&mut self, weight: Option<G::Score>) {
        self.weight = weight;
    }
    pub fn set_children(&mut self, children: Vec<GameNode<G>>) {
        self.children = children;
    }

    pub fn print_root(&self) {
        self.game.print();
    }
}

impl<G: Game> Debug for GameNode<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{:?}: ", self.weight);
        let spaces = "|  ".repeat((self.depth + 1) as usize);
        for child in &self.children {
            s += &format!("\n{}{:?}", spaces, child);
        }
        f.write_str(&s)
    }
}