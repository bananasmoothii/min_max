use crate::game::Game;
use crate::min_max::node::GameNode;

use crate::scalar::Scalar;


pub mod node;

impl<G: Game> GameNode<G> {
    pub fn explore_children(&mut self, bot_player: G::Player, max_depth: u32) {
        self.explore_children_recur(bot_player, max_depth);

        self.complete_weights(bot_player);
    }

    fn explore_children_recur(&mut self, bot_player: G::Player, max_depth: u32) {
        let is_full = self.game.is_full();
        let winner = self.game.get_winner();

        if self.depth() >= max_depth || is_full || winner.is_some() {
            self.set_weight(
                Some(match winner {
                    Some(winner) =>
                        if winner == bot_player { G::Score::MAX() } else { G::Score::MIN() },
                    None => self.game.get_score(bot_player),
                }));

            return;
        }

        let children: Vec<GameNode<G>> = self.game.possible_plays().iter()
            .map(|&input_coord| {
                let mut game = self.game.clone();
                game.play(bot_player, input_coord).unwrap(); // should not panic as input_coord is a possible play
                GameNode::new(game, self.depth() + 1, None, bot_player)
            }).collect();
        self.set_children(children);

        for child in self.children_mut() {
            child.explore_children_recur(bot_player, max_depth);
        }
    }

    fn complete_weights(&mut self, bot_player: G::Player) {
        if self.weight().is_some() {
            return;
        }
        if self.children().is_empty() {
            self.set_weight(Some(self.game.get_score(bot_player)));
            return;
        }
        for child in self.children_mut() {
            child.complete_weights(bot_player);
        }
        let children_weights = self.children().iter()
            .map(|child| child.weight().unwrap());

        self.set_weight(Some(if self.player_who_played != bot_player {
            children_weights.min().unwrap() // currently our turn, so we want to minimize what the opponent can do
        } else {
            children_weights.max().unwrap() // currently player's turn, so he wants to maximize his score
        }));
    }

    pub fn choose_best_child(&self) -> Option<&GameNode<G>> {
        if self.children().is_empty() {
            return None;
        }
        self.children().iter().find(|child| child.weight() == self.weight()) // TODO: use binary search and sort children by weight (maybe?)
    }
}
