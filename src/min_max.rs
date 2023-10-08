use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
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

    const FORK_DEPTH: u32 = 1;

    const USE_GAME_SCORE: bool = true;

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
            // weight should have been set by the functions above
            return self.weight().unwrap();
        }

        let maximize = now_playing == bot_player;

        let worst_child_score = Arc::new(Mutex::new(if !maximize {
            // inverting for children
            G::Score::MAX()
        } else {
            G::Score::MIN()
        }));

        // WARNING: (maybe) destroying game here, for memory efficiency

        let check_children = self.fill_children_and_destroy_game(now_playing, real_plays);

        let auto_destroy = AtomicBool::new(false);

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
                auto_destroy.store(true, Relaxed);
                None // abort checking childs
            } else {
                // parent may choose us
                Some(()) // continue checking children
            }
        };

        //let spaces = "| ".repeat(self.depth() as usize);
        //println!("{}({})Exploring {} children of depth {} (actual: {})...", spaces, self.id(), self.children.len(), self.depth(), self.depth().overflowing_sub(real_plays).0);

        //println!("({} - {real_plays} = {} )", self.depth(), self.depth().overflowing_sub(real_plays).0);
        let weight = if self.is_parallelize_depth(real_plays) {
            // parallelize
            self.children.par_iter_mut().try_for_each(|(_, child)| {
                print!("F");
                maybe_explore_children(child)
            });
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

        if auto_destroy.load(Relaxed) {
            self.children = Vec::with_capacity(0);
        }

        weight
    }

    fn is_parallelize_depth(&self, real_plays: u32) -> bool {
        // no need to check whether it overflows as there won't be u32::MAX plays
        self.depth().overflowing_sub(real_plays).0 == Self::FORK_DEPTH && Self::MULTI_THREADING
    }

    /// Returns true if childrens should be checked for win or draw, false if they were already checked
    fn fill_children_and_destroy_game(
        &mut self,
        now_playing: <G as Game>::Player,
        real_plays: u32,
    ) -> bool {
        let game = self.game.as_ref().expect("game was removed too early");

        if self.children.is_empty() {
            // if we take the game from us, we don't have to clone it, but we can do this only once
            let take_game = self.depth() != real_plays;

            let possible_plays = game.possible_plays();
            let possibilities = possible_plays.len();
            let mut vec = Vec::with_capacity(possibilities);

            if possibilities >= 2 || !take_game {
                for i in 0..(possibilities - 1) {
                    let input_coord = possible_plays[i];
                    let mut game = game.clone();
                    game.play(now_playing, input_coord).unwrap(); // should not panic as input_coord is a possible play
                    vec.push((
                        input_coord,
                        GameNode::new(
                            Some(game),
                            self.depth() + 1,
                            None,
                            PlayersTurn(now_playing.other(), Some(input_coord)),
                        ),
                    ));
                }
            }
            if possibilities >= 1 {
                let input_coord = possible_plays[possibilities - 1];
                let mut game = if take_game {
                    // we don't want to destroy the game in the root node
                    // add the game itself while "destroying" it for us
                    self.game.take().unwrap()
                } else {
                    game.clone()
                };
                game.play(now_playing, input_coord).unwrap();
                vec.push((
                    input_coord,
                    GameNode::new(
                        Some(game),
                        self.depth() + 1,
                        None,
                        PlayersTurn(now_playing.other(), Some(input_coord)),
                    ),
                ));
            }

            self.children = vec;
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
            let weight = Some(
                if Self::USE_GAME_SCORE {
                    // I know this is a constant, but this allows me to change it easily
                    let score = game.get_score(bot_player); // computing score here
                    if score == G::Score::MAX() {
                        self.game_state = self.game_state.to_win_by(bot_player);
                    } else if score == G::Score::MIN() {
                        self.game_state = self.game_state.to_win_by(bot_player.other());
                    }
                    score
                } else {
                    if let Some(winner) = game.get_winner() {
                        self.game_state = self.game_state.to_win_by(winner);
                        if winner == bot_player {
                            G::Score::MAX()
                        } else {
                            G::Score::MIN()
                        }
                    } else {
                        G::Score::ZERO()
                    }
                }
                .add_towards_0((self.depth() - real_plays) as i32), // we want to prioritize the fastest win
            );
            self.set_weight(weight);
            return true;
        }
        false
    }

    pub fn into_best_child(self) -> Self {
        let target_weight = self.weight().unwrap();
        let mut best_child = None;
        for (_, child) in self.children.into_iter() {
            if child.weight().unwrap() == target_weight {
                best_child = Some(child);
                break;
            }
        }
        let mut best_child = best_child.expect("No children found");
        best_child.fill_play(self.game.unwrap());
        best_child
        /*
                let mut best = G::Score::MIN();
                let mut best_child = None;
                for mut child in self.children.into_values() {
                    let child_weight = child.weight().unwrap();
                    if child_weight == target_weight {
                        child.fill_play(self.game.unwrap());
                        return child;
                    }
                    if child_weight > best {
                        best = child_weight;
                        best_child = Some(child);
                    }
                }
                println!("Could not find target weight {} in children", target_weight);
                let mut best_child = best_child.expect("No children found");
                best_child.fill_play(self.game.unwrap());
                best_child
        */
        /*
        let game = self.game
            .expect("game should not have been removed from the root node");

        let mut best_child = self
            .children // parallelization here slows downs the program a lot
            .into_iter()
            .max_by_key(|(_, child)| child.weight())
            .unwrap()
            .1;

        best_child.fill_play(game);
        best_child

         */
    }
}
