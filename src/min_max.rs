use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use rayon::iter::*;

use crate::game::player::Player;
use crate::game::state::GameState::*;
use crate::game::Game;
use crate::min_max::node::GameNode;
use crate::scalar::Scalar;

pub mod node;

impl<G: Game + Send + Sync> GameNode<G> {
    pub fn explore_children(&mut self, bot_player: G::Player, max_depth: u32, real_plays: u32) {
        let now_playing = match self.game_state {
            PlayersTurn(playing_player) => playing_player,
            _ => panic!(
                "Cannot explore children of a node that is not starting or played by a player"
            ),
        };

        println!("Exploring children...");

        self.explore_children_recur(
            bot_player,
            max_depth,
            now_playing,
            real_plays,
            self.children.is_empty(),
        );

        // println!("Completing weights...");
        // self.complete_weights(bot_player);
    }

    const FORK_DEPTH: u32 = 3;

    const USE_GAME_SCORE: bool = false;

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
    ) -> G::Score {
        assert!(self.depth() >= real_plays, "Negative exploration");

        let do_checks = checks || self.children.is_empty();

        if self.check_max_depth(bot_player, max_depth, real_plays)
            || (do_checks && (self.check_winner(bot_player) || self.check_draw()))
        {
            return if Self::USE_GAME_SCORE {
                self.game.get_score(bot_player)
            } else {
                if let Some(winner) = self.game.get_winner() {
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
        })); // TODO: seems like sometimes this is not changed

        let stop = AtomicBool::new(false);

        let check_children = self.fill_children(now_playing);

        let maybe_explore_children = |child: &mut Self| {
            if stop.load(Ordering::Relaxed) {
                return;
            }
            let child_score = child.explore_children_recur(
                bot_player,
                max_depth,
                now_playing.other(),
                real_plays,
                check_children,
            );
            let mut worst_child_score = worst_child_score.lock().unwrap();
            // println!("maximize: {maximize},  child: {child_score} worst child: {worst_child_score}");

            if (maximize && child_score > *worst_child_score) // we found better than the child sibling, but the node above will choose the worst so we are basically useless
                || (!maximize && child_score < *worst_child_score)
            {
                // stop.store(true, Ordering::Relaxed);
                *worst_child_score = child_score;
            }
        };

        //let spaces = "| ".repeat(self.depth() as usize);
        //println!("{}({})Exploring {} children of depth {} (actual: {})...", spaces, self.id(), self.children.len(), self.depth(), self.depth().overflowing_sub(real_plays).0);

        //println!("({} - {real_plays} = {} )", self.depth(), self.depth().overflowing_sub(real_plays).0);
        if self.depth().overflowing_sub(real_plays).0 == Self::FORK_DEPTH && false {
            // parallelize
            print!("F"); // should print 49 F (7^(FORK_DEPTH-1) = 7^2)
            self.children.par_iter_mut().for_each(|(_, child)| {
                maybe_explore_children(child);
            });
            let weight = (*worst_child_score.lock().unwrap()).into();
            self.set_weight(weight);
            weight.unwrap()
        } else {
            self.children.iter_mut().for_each(|(_, child)| {
                maybe_explore_children(child);
            });
            let weight = (*worst_child_score.lock().unwrap()).into();
            self.set_weight(weight);
            weight.unwrap()
        }
    }

    fn fill_children(&mut self, now_playing: <G as Game>::Player) -> bool {
        if self.children.is_empty() {
            self.children = self
                .game
                .possible_plays()
                .iter()
                .map(|&input_coord| {
                    let mut game = self.game.clone();
                    game.play(now_playing, input_coord).unwrap(); // should not panic as input_coord is a possible play

                    (
                        input_coord,
                        GameNode::new(
                            game,
                            self.depth() + 1,
                            None,
                            PlayersTurn(now_playing.other()),
                        ),
                    )
                })
                .collect();
            true
        } else {
            false
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
            self.set_weight(Some(if winner == bot_player {
                G::Score::MAX()
            } else {
                G::Score::MIN()
            }));
            self.game_state = WonBy(winner);
            return true;
        }
        false
    }

    //noinspection RsConstantConditionIf
    fn check_max_depth(
        &mut self,
        bot_player: <G as Game>::Player,
        max_depth: u32,
        real_plays: u32,
    ) -> bool {
        if self.depth() >= max_depth + real_plays {
            // I know this is a constant, but this allows me to change it easily
            if Self::USE_GAME_SCORE {
                let score = self.game.get_score(bot_player); // computing score here
                if score == G::Score::MAX() {
                    self.game_state = WonBy(bot_player);
                    self.set_weight(Some(
                        score.add_towards_0((self.depth() - real_plays) as i32),
                    ))
                } else if score == G::Score::MIN() {
                    self.game_state = WonBy(bot_player.other());
                    self.set_weight(Some(
                        score.add_towards_0((self.depth() - real_plays) as i32),
                    ))
                }
            } else {
                if let Some(winner) = self.game.get_winner() {
                    self.set_weight(Some(
                        (if winner == bot_player {
                            G::Score::MAX()
                        } else {
                            G::Score::MIN()
                        })
                        .add_towards_0((self.depth() - real_plays) as i32),
                    ));
                    self.game_state = WonBy(winner);
                } else {
                    self.set_weight(Some(G::Score::ZERO()));
                }
            }
            return true;
        }
        false
    }

    pub fn into_best_child(self) -> Self {
        //let target_weight = self.weight().unwrap();
        self.children // parallelization here slows downs the program a lot
            .into_iter()
            .max_by_key(|(_, child)| child.weight())
            .unwrap()
            .1
    }
}
