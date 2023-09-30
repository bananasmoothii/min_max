use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::game::state::GameState;
use crate::game::Game;
use crate::game::state::GameState::PlayersTurn;

pub struct GameNode<G: Game> {
    depth: u32,
    weight: Option<G::Score>,
    pub(super) children: HashMap<G::InputCoordinate, Self>,
    pub game: G,
    pub game_state: GameState<G>,
}

impl<G: Game> GameNode<G> {
    pub fn new(game: G, depth: u32, weight: Option<G::Score>, game_state: GameState<G>) -> Self {
        GameNode {
            depth,
            weight,
            children: HashMap::new(),
            game,
            game_state,
        }
    }

    pub fn new_root(game: G, starting_player: G::Player) -> Self {
        GameNode::new(game, 0, None, PlayersTurn(starting_player))
    }

    /**
     * Returns (false, self) if the child does not exist, else (true, child)
     */
    pub fn try_into_child(mut self, play: G::InputCoordinate) -> (bool, Self) {
        if let Some(child) = self.children.remove(&play) {
            (true, child)
        } else {
            (false, self)
        }
    }

    /**
     * Same as try_into_child, but if the child does not exist, you don't get the ownership of self back
     */
    pub fn into_child(mut self, play: G::InputCoordinate) -> Option<Self> {
        self.children.remove(&play)
    }

    // Getters

    pub fn depth(&self) -> u32 {
        self.depth
    }
    pub fn weight(&self) -> Option<G::Score> {
        self.weight
    }
    pub fn children(&self) -> &HashMap<G::InputCoordinate, Self> {
        &self.children
    }
    pub fn children_mut(&mut self) -> &mut HashMap<G::InputCoordinate, Self> {
        &mut self.children
    }

    // Setters

    pub fn set_weight(&mut self, weight: Option<G::Score>) {
        self.weight = weight;
    }
    pub fn set_children(&mut self, children: HashMap<G::InputCoordinate, Self>) {
        self.children = children;
    }

    pub fn print_root(&self) {
        self.game.print();
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
            return s
        }
        let spaces = "|  ".repeat((self.depth + 1) as usize);
        for (input, child) in &self.children {
            s += &format!("\n{spaces}({}) {input} scores {}", self.game_state, child.debug(max_depth - 1));
        }
        s
    }

    fn count_depth(&self) -> u32 {
        let mut max_depth = 0;
        for child in self.children.values() {
            let child_depth = child.count_depth();
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
        for (input, child) in &self.children {
            s += &format!("\n{spaces}({}) {input} scores {child:?}", self.game_state);
        }
        f.write_str(&s)
    }
}