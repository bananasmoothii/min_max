use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::game::state::GameState;
use crate::game::state::GameState::PlayersTurn;
use crate::game::Game;

#[derive(Clone)]
pub struct GameNode<G: Game> {
    depth: u32,
    weight: Option<G::Score>,
    pub(super) children: HashMap<G::InputCoordinate, Self>,
    pub(super) game: Option<G>,
    pub game_state: GameState<G>,
}

impl<G: Game> GameNode<G> {
    pub fn new(
        game: Option<G>,
        depth: u32,
        weight: Option<G::Score>,
        game_state: GameState<G>,
    ) -> Self {
        Self {
            depth,
            weight,
            children: HashMap::new(),
            game,
            game_state,
        }
    }

    pub fn new_root(game: G, starting_player: G::Player, depth: u32) -> Self {
        GameNode::new(Some(game), depth, None, PlayersTurn(starting_player, None))
    }

    pub fn fill_play(&mut self, mut previous_game: G) {
        if self.game.is_some() {
            return;
        }
        let (last_player, last_play) = self.game_state.get_last_play();
        previous_game.play(last_player, last_play.unwrap()).unwrap();
        self.game = Some(previous_game);
    }

    pub fn regenerate_children_games(&mut self) {
        let game = self.game.as_ref().unwrap();
        for (_, child) in &mut self.children {
            child.fill_play(game.clone());
        }
    }

    /**
     * Returns (false, self) if the child does not exist, else (true, child)
     */
    pub fn try_into_child(mut self, play: G::InputCoordinate) -> (bool, Self) {
        if let Some(mut child) = self.children.remove(&play) {
            child.fill_play(self.game.unwrap());
            (true, child)
        } else {
            (false, self)
        }
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

    pub fn game(&self) -> Option<&G> {
        self.game.as_ref()
    }

    pub fn expect_game(&self) -> &G {
        self.game.as_ref().expect("GameNode has no game")
    }

    pub fn expect_game_mut(&mut self) -> &mut G {
        self.game.as_mut().expect("GameNode has no game")
    }

    pub fn into_expect_game(self) -> G {
        self.game.expect("GameNode has no game")
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
        for (input, child) in &self.children {
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
        for child in self.children.values() {
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
        for (input, child) in &self.children {
            s += &format!("\n{spaces}({}) {input} scores {child:?}", self.game_state);
        }
        f.write_str(&s)
    }
}
