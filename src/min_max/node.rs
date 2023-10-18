use std::fmt::{Debug, Formatter};

use crate::game::state::GameState;
use crate::game::state::GameState::PlayersTurn;
use crate::game::Game;

#[derive(Clone)]
pub struct GameNode<G: Game> {
    depth: u32,
    weight: Option<G::Score>,
    pub(super) children: Vec<(G::InputCoordinate, Self)>,
    pub(super) game: G,
    pub game_state: GameState<G>,
}

impl<G: Game> GameNode<G> {
    pub fn new(game: G, depth: u32, weight: Option<G::Score>, game_state: GameState<G>) -> Self {
        Self {
            depth,
            weight,
            children: Vec::new(),
            game,
            game_state,
        }
    }

    pub fn new_root(game: G, starting_player: G::Player, depth: u32) -> Self {
        GameNode::new(game, depth, None, PlayersTurn(starting_player, None))
    }

    /**
     * Returns (false, self) if the child does not exist, else (true, child)
     */
    pub fn try_into_child(mut self, play: G::InputCoordinate) -> (bool, Self) {
        let mut new_children = Vec::with_capacity(self.children.len());
        for (coord, mut child) in self.children.into_iter() {
            if coord == play {
                return (true, child);
            }
            new_children.push((coord, child));
        }
        self.children = new_children;
        (false, self)
    }

    // Getters

    pub fn depth(&self) -> u32 {
        self.depth
    }
    pub fn weight(&self) -> Option<G::Score> {
        self.weight
    }
    pub fn children(&self) -> &Vec<(G::InputCoordinate, Self)> {
        &self.children
    }

    pub fn game(&self) -> &G {
        &self.game
    }

    pub fn expect_game_mut(&mut self) -> &mut G {
        &mut self.game
    }

    pub fn into_game(self) -> G {
        self.game
    }

    // Setters

    pub fn set_weight(&mut self, weight: Option<G::Score>) {
        self.weight = weight;
    }

    pub fn debug(&self, max_depth: u32) -> String {
        let weight_str = if let Some(weight) = self.weight {
            weight.to_string()
        } else {
            "?".to_string()
        };
        let mut s = format!("{weight_str}: ");
        if max_depth == 0 {
            let depth = self.count_depth();
            if depth > 0 {
                s.push_str(&*format!("{} non shown", depth));
            }
            return s;
        }
        let spaces = "|  ".repeat((self.depth + 1) as usize);
        for (input, child) in self.children.iter() {
            s += &format!(
                "\n{spaces}({}) {input} scores {}",
                self.game_state,
                child.debug(max_depth - 1)
            );
        }
        s
    }

    fn count_depth(&self) -> u32 {
        let mut max_depth = 0;
        for (_, child) in self.children.iter() {
            let child_depth = child.count_depth() + 1;
            if child_depth > max_depth {
                max_depth = child_depth;
            }
        }
        max_depth
    }
}

impl<G: Game> Debug for GameNode<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let weight_str = if let Some(weight) = self.weight {
            weight.to_string()
        } else {
            "?".to_string()
        };
        let mut s = format!("{weight_str}: ");
        let spaces = "|  ".repeat((self.depth + 1) as usize);
        for (input, child) in self.children.iter() {
            s += &format!("\n{spaces}({}) {input} scores {child:?}", self.game_state);
        }
        f.write_str(&s)
    }
}
