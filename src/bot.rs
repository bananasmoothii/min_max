use crate::game::player::Player;
use crate::game::Game;
use crate::min_max::node::GameNode;
use crate::scalar::Scalar;

pub struct Bot<G: Game> {
    player: G::Player,
    game_tree: Option<GameNode<G>>,
    max_depth: u32,
    times: Vec<u64>,
}

impl<G: Game> Bot<G> {
    pub fn new(player: G::Player, max_depth: u32) -> Self {
        Self {
            player,
            /// game_tree should never be None
            game_tree: Some(GameNode::new_root(G::new(), player, 0)),
            max_depth,
            times: Vec::new(),
        }
    }

    pub fn other_played(&mut self, play: G::InputCoordinate) -> Result<(), &str> {
        let had_children = self
            .game_tree
            .as_ref()
            .is_some_and(|tree| !tree.children().is_empty());
        let (is_known_move, mut new_game_tree) = self
            .game_tree
            .take()
            .unwrap_or_else(|| panic!("game_tree should never be none, object is invalid"))
            .try_into_child(play);
        if is_known_move {
            self.game_tree = Some(new_game_tree);
            debug_assert!(self
                .game_tree
                .as_ref()
                .is_some_and(|tree| tree.game().is_some()));
        } else {
            // Here, new_game_tree is actually game_tree, the ownership was given back to us
            let result = new_game_tree
                .expect_game_mut()
                .play(self.player.other(), play);
            if let Err(err) = result {
                self.game_tree = Some(new_game_tree);
                return Err(err);
            }
            if had_children {
                println!("Unexpected move... Maybe you are a pure genius, or a pure idiot.");
            }
            let depth = new_game_tree.depth() + 1;
            let game = new_game_tree.into_expect_game();
            self.game_tree = Some(GameNode::new_root(game, self.player, depth));
        }
        Ok(())
    }

    pub fn play(&mut self) -> G::InputCoordinate {
        let start = std::time::Instant::now();
        let game_tree = self
            .game_tree
            .as_mut()
            .expect("Bot has not been initialized");
        game_tree.explore_children(
            self.player,
            self.max_depth,
            game_tree.expect_game().plays() as u32,
        );
        // println!("Tree:\n {}", game_tree.debug(2));
        println!("Comparing possibilities...");
        self.game_tree = Some(self.game_tree.take().unwrap().into_best_child());

        let game_tree = self.game_tree.as_ref().unwrap();

        let time = start.elapsed().as_millis() as u64;
        self.times.push(time);
        println!("Done in {}ms", time);

        let weight_opt = game_tree.weight();
        if weight_opt.is_some_and(|it| it > G::Score::MAX().add_towards_0(1000)) {
            println!("You're dead, sorry.");
        } else if weight_opt.is_some_and(|it| it < G::Score::MIN().add_towards_0(1000)) {
            println!("Ok I'm basically dead...");
        }

        debug_assert!(game_tree.game().is_some());
        game_tree.game_state.get_last_play().1.unwrap()
    }

    pub fn average_time(&self) -> u64 {
        self.times.iter().sum::<u64>() / self.times.len() as u64
    }

    pub fn expect_game(&self) -> &G {
        self.game_tree.as_ref().unwrap().expect_game()
    }
}
