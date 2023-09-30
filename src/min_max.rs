use std::collections::HashMap;
use crate::game::Game;
use crate::game::player::Player;
use crate::game::state::GameState::{Draw, PlayersTurn, WonBy};
use crate::min_max::node::GameNode;

use crate::scalar::Scalar;


pub mod node;

impl<G: Game> GameNode<G> {
    pub fn explore_children(&mut self, bot_player: G::Player, max_depth: u32, real_plays: u32) {
        self.explore_children_recur(bot_player, max_depth, match self.game_state {
            PlayersTurn(playing_player) => playing_player,
            _ => panic!("Cannot explore children of a node that is not starting or played by a player"),
        }, real_plays);

        self.complete_weights(bot_player);
    }

    fn explore_children_recur(&mut self, bot_player: G::Player, max_depth: u32, now_playing: G::Player, real_plays: u32) {
        if self.check_max_depth(bot_player, max_depth, real_plays) { return; }

        if self.check_winner(bot_player) { return; }

        if self.check_draw() { return; }

        self.fill_children(now_playing);
        for child in self.children_mut().values_mut() {
            child.explore_children_recur(bot_player, max_depth, now_playing.other(), real_plays);
        }
    }

    fn fill_children(&mut self, now_playing: <G as Game>::Player) {
        if self.children.is_empty() {
            let children: HashMap<G::InputCoordinate, Self> = self.game.possible_plays().iter()
                .map(|&input_coord| {
                    let mut game = self.game.clone();
                    game.play(now_playing, input_coord).unwrap(); // should not panic as input_coord is a possible play

                    (input_coord, GameNode::new(game, self.depth() + 1, None, PlayersTurn(now_playing.other())))
                })
                .collect();

            self.children = children;
        }
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

    const USE_GAME_SCORE: bool = false;

    //noinspection RsConstantConditionIf
    fn check_max_depth(&mut self, bot_player: <G as Game>::Player, max_depth: u32, real_plays: u32) -> bool {
        if self.depth() >= max_depth + real_plays {
            if Self::USE_GAME_SCORE { // I know this is a constant, but this allows me to change it easily
                let score = self.game.get_score(bot_player); // computing score here
                if score == G::Score::MAX() {
                    self.game_state = WonBy(bot_player);
                    self.set_weight(Some(score.add_towards_0((self.depth() - real_plays) as i32)))
                } else if score == G::Score::MIN() {
                    self.game_state = WonBy(bot_player.other());
                    self.set_weight(Some(score.add_towards_0((self.depth() - real_plays) as i32)))
                }
            } else {
                if let Some(winner) = self.game.get_winner() {
                    self.set_weight(Some((if winner == bot_player { G::Score::MAX() } else { G::Score::MIN() }).add_towards_0((self.depth() - real_plays) as i32)));
                    self.game_state = WonBy(winner);
                } else {
                    self.set_weight(Some(G::Score::ZERO()));
                }
            }
            return true;
        }
        false
    }

    fn complete_weights(&mut self, bot_player: G::Player) {
        if self.children().is_empty() {
            if self.weight().is_none() {
                panic!("there should be a weight for a leaf node");
            }
            return;
        }
        for child in self.children_mut().values_mut() {
            child.complete_weights(bot_player);
        }

        let children_weights = self.children().values()
            .map(|child| child.weight().unwrap()); // the tree should be completed, so no None should be found

        let now_playing = match self.game_state {
            PlayersTurn(now_playing) => now_playing,
            _ => panic!("Cannot complete weights of a node that is not starting or played by a player"),
        };

        self.set_weight(Some(if now_playing == bot_player {
            // currently bot's turn and he will play the best move for him
            children_weights.max().unwrap()
        } else {
            // currently opponent's turn and he will play the best move for him, so the worst for us
            children_weights.min().unwrap()
        }))
    }

    /*
    pub fn choose_best_child_mut(&mut self) -> Option<&mut Self> {
        if self.children().is_empty() {
            return None;
        }
        let weight = self.weight();
        self.children_mut().iter_mut().find(|child| child.weight() == weight)
    }
     */

    pub fn into_best_child(self) -> Self {
        self.children.into_values()
            .max_by_key(|child| child.weight())
            .unwrap()
    }

    /*
        pub fn play_and_get_played_node<'a>(&'a mut self, player: G::Player, input_coord: G::InputCoordinate) -> Result<&'a mut Self, &str> {
            let new_node = if let Some(new_node) = self.get_node_with_play_mut(input_coord) {
                new_node // no need to play, play is already done
            } else {
                println!("Unpredicted move");
                let result = self.game.clone().play(player, input_coord);
                if let Err(e) = result {
                    return Err(e);
                }
                let new_node = GameNode::new(
                    self.game.clone(),
                    self.depth() + 1,
                    None,
                    PlayersTurn(player.other()),
                );
                self.children_mut().push(new_node); // add it to tree, we can't own it because we need its reference
                self.children_mut().last_mut().unwrap()
            };
            Ok(new_node)
        }

     */
}
