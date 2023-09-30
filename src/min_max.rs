use crate::game::Game;
use crate::game::player::Player;
use crate::game::state::GameState::{Draw, PlayersTurn, WonBy};
use crate::min_max::node::GameNode;

use crate::scalar::Scalar;


pub mod node;

impl<G: Game> GameNode<G> {
    pub fn explore_children(&mut self, bot_player: G::Player, max_depth: u32) {
        self.explore_children_recur(bot_player, max_depth, match self.game_state {
            PlayersTurn(playing_player) => playing_player,
            _ => panic!("Cannot explore children of a node that is not starting or played by a player"),
        });

        self.complete_weights(bot_player);
    }

    fn explore_children_recur(&mut self, bot_player: G::Player, max_depth: u32, now_playing: G::Player) {
        if self.check_max_depth(bot_player, max_depth) { return; }

        if self.check_winner(bot_player) { return; }

        if self.check_draw() { return; }

        self.fill_children(now_playing);
        for child in self.children_mut() {
            child.explore_children_recur(bot_player, max_depth, now_playing.other());
        }
    }

    fn fill_children(&mut self, now_playing: <G as Game>::Player) {
        let children: Vec<GameNode<G>> = self.game.possible_plays().iter()
            .map(|&input_coord| {
                let mut game = self.game.clone();
                game.play(now_playing, input_coord).unwrap(); // should not panic as input_coord is a possible play

                GameNode::new(game, self.depth() + 1, None, PlayersTurn(now_playing))
            })
            .collect();

        self.set_children(children);
    }

    fn check_draw(&mut self) -> bool {
        if self.game.is_full() {
            // consider draw as a loss for the bot, but not a loss as important as a real loss
            let half_loose = G::Score::MIN().div(2);
            self.set_weight(Some(half_loose));
            self.game_state = Draw;
            return true;
        }
        false
    }

    fn check_winner(&mut self, bot_player: <G as Game>::Player) -> bool {
        let winner = self.game.get_winner();
        if let Some(winner) = winner {
            self.set_weight(Some(if winner == bot_player { G::Score::MAX() } else { G::Score::MIN() }));
            self.game_state = WonBy(winner);
            return true;
        }
        false
    }

    fn check_max_depth(&mut self, bot_player: <G as Game>::Player, max_depth: u32) -> bool {
        if self.depth() >= max_depth {
            let score = self.game.get_score(bot_player); // computing score here
            self.set_weight(Some(score));
            if score == G::Score::MAX() {
                self.game_state = WonBy(bot_player);
            } else if score == G::Score::MIN() {
                self.game_state = WonBy(bot_player.other());
            }
            return true;
        }
        false
    }

    fn complete_weights(&mut self, bot_player: G::Player) {
        if self.children().is_empty() {
            if self.weight().is_none() {
                self.set_weight(Some(self.game.get_score(bot_player)));
            }
            return;
        }
        for child in self.children_mut() {
            child.complete_weights(bot_player);
        }

        let children_weights = self.children().iter()
            .map(|child| child.weight().unwrap()); // the tree should be completed, so no None should be found

        let now_playing = match self.game_state {
            PlayersTurn(now_playing) => now_playing,
            _ => panic!("Cannot complete weights of a node that is not starting or played by a player"),
        };

        self.set_weight(if now_playing == bot_player {
            children_weights.min() // we want to minimize the choices of the opponent
        } else {
            children_weights.max() // currently opponent's turn and he will probably play the best move for him
        })
    }

    pub fn choose_best_child_mut(&mut self) -> Option<&mut GameNode<G>> {
        if self.children().is_empty() {
            return None;
        }
        let weight = self.weight();
        self.children_mut().iter_mut().find(|child| child.weight() == weight) // TODO: use binary search and sort children by weight (maybe?)
    }

    pub fn get_node_with_play_mut(&mut self, play: G::InputCoordinate) -> Option<&mut GameNode<G>> {
        for child in self.children_mut() {
            if child.game.last_play().unwrap() == play {
                return Some(child);
            }
        }
        None
    }
}
