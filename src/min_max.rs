use std::sync::{Arc, Mutex};

use rayon::iter::*;

use crate::game::player::Player;
use crate::game::state::GameState::*;
use crate::game::Game;
use crate::min_max::node::GameNode;
use crate::scalar::Scalar;

pub mod node;

impl<G: Game> GameNode<G> {
    pub fn explore_children(&mut self, bot_player: G::Player, max_depth: u32, real_plays: u32) {
        let now_playing = match self.game_state {
            PlayersTurn(playing_player, _) => playing_player,
            _ => panic!(
                "Cannot explore children of a node that is not starting or played by a player. Current state: {}",
                self.game_state
            ),
        };

        println!("Exploring possibilities...");

        self.explore_children_recur(
            bot_player,
            max_depth,
            now_playing,
            real_plays,
            self.children.is_empty(),
            Arc::new(Mutex::new(if now_playing == bot_player {
                G::Score::MAX()
            } else {
                G::Score::MIN()
            })),
        );

        // println!("Completing weights...");
        // self.complete_weights(bot_player);
    }

    const FORK_DEPTH: u32 = 2;

    const USE_GAME_SCORE: bool = false;

    const MULTI_THREADING: bool = true;

    /// Explore children recursively
    ///
    /// # Parameters
    /// * `real_plays` - number of plays made that were actually made, not just predicted
    /// * `checks` - if true, will check if the game is won or draw, else will assume it is not
    /// * `worst_sibling_score` - if now_playing is the bot, the minimum score to consider, because
    ///    we are maximizing children, else the maximum score to consider because we are minimizing
    ///    children
    ///
    /// # Returns
    /// The weight of the node
    fn explore_children_recur(
        &mut self,
        bot_player: G::Player,
        max_depth: u32,
        now_playing: G::Player,
        real_plays: u32,
        checks: bool,
        worst_sibling_score: Arc<Mutex<G::Score>>,
    ) -> G::Score {
        assert!(self.depth() >= real_plays, "Negative exploration");

        let do_checks = checks || self.children.is_empty();

        if self.check_max_depth(bot_player, max_depth, real_plays)
            || (do_checks && (self.check_winner(bot_player) || self.check_draw()))
        {
            let game = self.game.as_ref().expect("game was removed too early");
            return if Self::USE_GAME_SCORE {
                game.get_score(bot_player)
            } else {
                if let Some(winner) = game.get_winner() {
                    if winner == bot_player {
                        G::Score::MAX()
                    } else {
                        G::Score::MIN()
                    }
                    .add_towards_0(self.depth() as i32 - real_plays as i32)
                } else {
                    G::Score::ZERO()
                }
            };
        }

        let maximize = now_playing == bot_player;

        let worst_child_score = Arc::new(Mutex::new(if !maximize {
            // inverting for children
            G::Score::MAX()
        } else {
            G::Score::MIN()
        }));

        let check_children = self.fill_children(now_playing);

        let maybe_explore_children = |child: &mut Self| {
            let child_score = child.explore_children_recur(
                bot_player,
                max_depth,
                now_playing.other(),
                real_plays,
                check_children,
                worst_child_score.clone(),
            );
            let mut worst_child_score = worst_child_score.lock().unwrap();
            // println!("maximize: {maximize},  child: {child_score} worst child: {worst_child_score}");

            if (maximize && child_score > *worst_child_score)
                || (!maximize && child_score < *worst_child_score)
            {
                *worst_child_score = child_score;
            }
            let parent_maximize = !maximize;
            let worst_sibling_score = worst_sibling_score.lock().unwrap();

            // if the parent will not choose us
            if (parent_maximize && *worst_child_score < *worst_sibling_score)
                || (!parent_maximize && *worst_child_score > *worst_sibling_score)
            {
                None // abort checking childs
            } else {
                // parent may choose us
                Some(()) // continue checking children
            }
        };

        //let spaces = "| ".repeat(self.depth() as usize);
        //println!("{}({})Exploring {} children of depth {} (actual: {})...", spaces, self.id(), self.children.len(), self.depth(), self.depth().overflowing_sub(real_plays).0);

        //println!("({} - {real_plays} = {} )", self.depth(), self.depth().overflowing_sub(real_plays).0);
        let weight = if self.depth().overflowing_sub(real_plays).0 == Self::FORK_DEPTH
            && Self::MULTI_THREADING
        {
            // parallelize
            self.children
                .par_iter_mut()
                .try_for_each(|(_, child)| maybe_explore_children(child));
            let weight = (*worst_child_score.lock().unwrap()).into();
            self.set_weight(weight);
            weight.unwrap()
        } else {
            self.children
                .iter_mut()
                .try_for_each(|(_, child)| maybe_explore_children(child));
            let weight = (*worst_child_score.lock().unwrap()).into();
            self.set_weight(weight);
            weight.unwrap()
        };

        // WARNING: (maybe) destroying game here, for memory efficiency
        if self.depth() != real_plays {
            self.game = None;
        }

        weight
    }

    fn fill_children(&mut self, now_playing: <G as Game>::Player) -> bool {
        let game = self.game.as_ref().expect("game was removed too early");

        if self.children.is_empty() {
            self.children = game
                .possible_plays()
                .iter()
                .map(|&input_coord| {
                    let mut game = game.clone();
                    game.play(now_playing, input_coord).unwrap(); // should not panic as input_coord is a possible play

                    (
                        input_coord,
                        GameNode::new(
                            Some(game),
                            self.depth() + 1,
                            None,
                            PlayersTurn(now_playing.other(), Some(input_coord)),
                        ),
                    )
                })
                .collect(); // todo: a lot of memory is used here
            true
        } else {
            self.regenerate_children_games();
            false
        }
    }

    fn check_draw(&mut self) -> bool {
        // consider draw as a loss for the bot, but not a loss as important as a real loss
        let half_loose = G::Score::MIN().div(2);
        if let Draw(_, _) = self.game_state {
            // if we are here, it means that this function was called twice on the same node
            self.expect_game().print();
            self.set_weight(Some(half_loose));
            return true;
        }
        if self.game.as_ref().unwrap().is_full() {
            self.set_weight(Some(half_loose));
            self.game_state = self.game_state.to_draw();
            return true;
        }
        false
    }

    fn check_winner(&mut self, bot_player: <G as Game>::Player) -> bool {
        if let WonBy(winner, _) = self.game_state {
            self.set_weight(Self::win_weight(winner, bot_player));
            return true;
        }
        let winner = self.game.as_ref().unwrap().get_winner();
        if let Some(winner) = winner {
            self.set_weight(Self::win_weight(winner, bot_player));
            self.game_state = self.game_state.to_win();
            return true;
        }
        false
    }

    fn win_weight(
        winner: <G as Game>::Player,
        bot_player: <G as Game>::Player,
    ) -> Option<<G as Game>::Score> {
        Some(if winner == bot_player {
            G::Score::MAX()
        } else {
            G::Score::MIN()
        })
    }

    //noinspection RsConstantConditionIf
    fn check_max_depth(
        &mut self,
        bot_player: <G as Game>::Player,
        max_depth: u32,
        real_plays: u32,
    ) -> bool {
        let game = self.game.as_ref().expect("game was removed too early");
        if self.depth() >= max_depth + real_plays {
            // I know this is a constant, but this allows me to change it easily
            if Self::USE_GAME_SCORE {
                let score = game.get_score(bot_player); // computing score here
                if score == G::Score::MAX() {
                    self.game_state = self.game_state.to_win_by(bot_player);
                    self.set_weight(Some(
                        score.add_towards_0((self.depth() - real_plays) as i32),
                    ))
                } else if score == G::Score::MIN() {
                    self.game_state = self.game_state.to_win_by(bot_player.other());
                    self.set_weight(Some(
                        score.add_towards_0((self.depth() - real_plays) as i32),
                    ))
                }
            } else {
                if let Some(winner) = game.get_winner() {
                    self.set_weight(Some(
                        (if winner == bot_player {
                            G::Score::MAX()
                        } else {
                            G::Score::MIN()
                        })
                        .add_towards_0((self.depth() - real_plays) as i32),
                    ));
                    self.game_state = self.game_state.to_win_by(winner);
                } else {
                    self.set_weight(Some(G::Score::ZERO()));
                }
            }
            return true;
        }
        false
    }

    pub fn into_best_child(self) -> Self {
        /*
        let target_weight = self.weight().unwrap();
        let mut best = G::Score::MIN();
        let mut best_child = None;
        for child in self.children {
            let child_weight = child.1.weight().unwrap();
            if child_weight == target_weight {
                return child.1;
            }
            if child_weight > best {
                best = child_weight;
                best_child = Some(child.1);
            }
        }
        best_child.unwrap()
        */
        let game = self
            .game
            .expect("game should not have been removed from the root node");

        let mut best_child = self
            .children // parallelization here slows downs the program a lot
            .into_iter()
            .max_by_key(|(_, child)| child.weight())
            .unwrap()
            .1;

        best_child.fill_play(game);
        best_child
    }
}
