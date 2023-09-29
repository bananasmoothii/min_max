use std::fmt::{Debug, Formatter};
use crate::game::Game;
use crate::game::player::Player;

pub struct GameNode<G: Game> {
    depth: u32,
    weight: Option<G::Score>,
    children: Vec<GameNode<G>>,
    pub game: G,
    pub player_who_played: G::Player,
}

impl<G: Game> GameNode<G> {
    pub fn new(game: G, depth: u32, weight: Option<G::Score>, player_who_played: G::Player) -> GameNode<G> {
        GameNode {
            depth,
            weight,
            children: Vec::new(),
            game,
            player_who_played,
        }
    }

    pub fn new_root(game: G, beginning_player: G::Player) -> GameNode<G> {
        GameNode::new(game, 0, None, beginning_player.complementary())
    }

    // Getters

    pub fn depth(&self) -> u32 {
        self.depth
    }
    pub fn weight(&self) -> Option<G::Score> {
        self.weight
    }
    pub fn children_mut(&mut self) -> &mut Vec<GameNode<G>> {
        &mut self.children
    }

    pub fn children(&self) -> &Vec<GameNode<G>> {
        &self.children
    }

    // Setters

    pub fn set_weight(&mut self, weight: Option<G::Score>) {
        self.weight = weight;
    }
    pub fn set_children(&mut self, children: Vec<GameNode<G>>) {
        self.children = children;
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