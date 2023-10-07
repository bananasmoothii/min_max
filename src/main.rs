use std::io;
use std::num::NonZeroU8;

use crate::game::connect4::Power4;
use crate::game::player::Player;
use crate::game::Game;
use crate::min_max::node::GameNode;

mod game;
mod min_max;
mod scalar;

fn main() {
    let max_depth = 8;

    let mut times: Vec<u128> = Vec::new();

    let p1 = NonZeroU8::new(1).unwrap();
    let p2 = NonZeroU8::new(2).unwrap();

    let bot_player: NonZeroU8 = p2;

    let mut current_player = if ask_start() { p1 } else { p2 };

    let mut game_tree: GameNode<Power4> = GameNode::new_root(Power4::new(), current_player, 0);

    let bot_vs_bot = false; // doesn't work properly yet
    let mut game_tree_2 = GameNode::new_root(Power4::new(), current_player.other(), 0);

    let mut p1_score: i32 = 0;
    loop {
        println!();
        game_tree.expect_game().print();
        println!("Scores: {p1_score} for player 1");
        println!();
        println!("Player {current_player}'s turn");
        if current_player == bot_player {
            game_tree = bot_play(max_depth, &mut times, bot_player, game_tree);
            game_tree_2 = GameNode::new_root(
                game_tree.expect_game().clone(),
                current_player,
                game_tree.depth(),
            );
        } else {
            if bot_vs_bot {
                game_tree_2 = bot_play(max_depth, &mut times, bot_player.other(), game_tree_2);
                game_tree = GameNode::new_root(
                    game_tree_2.expect_game().clone(),
                    current_player,
                    game_tree_2.depth(),
                )
            } else {
                let (cont, tree) = player_play(current_player, game_tree);
                game_tree = tree;
                if cont {
                    continue;
                }
            }
        }

        let game = game_tree.expect_game();

        p1_score = game.get_score(p1);

        if p1_score == <Power4 as Game>::Score::MAX || p1_score == <Power4 as Game>::Score::MIN {
            println!("Player {current_player} won!\n");
            game.print();
            break;
        }
        if game.is_full() {
            println!("Draw!\n");
            game.print();
            break;
        }
        current_player = current_player.other();
    }
    println!(
        "Average time: {}ms",
        times.iter().sum::<u128>() / times.len() as u128
    );
}

fn player_play(
    current_player: NonZeroU8,
    mut game_tree: GameNode<Power4>,
) -> (bool, GameNode<Power4>) {
    let column = get_user_input();
    let had_children = !game_tree.children().is_empty();
    let (is_known_move, mut new_game_tree) = game_tree.try_into_child(column - 1);
    if is_known_move {
        game_tree = new_game_tree;
        debug_assert!(game_tree.game().is_some());
    } else {
        // Here, new_game_tree is actually game_tree, the ownership was given back to us
        if had_children {
            println!("Unexpected move... Maybe you are a pure genius, or a pure idiot.");
        }
        let result = new_game_tree
            .expect_game_mut()
            .play(current_player, column - 1);
        if let Err(e) = result {
            println!("{}", e);
            game_tree = new_game_tree;
            return (true, game_tree);
        }
        let depth = new_game_tree.depth() + 1;
        let game = new_game_tree.into_expect_game();
        let next_player = current_player.other();
        game_tree = GameNode::new_root(game, next_player, depth);
    }
    (false, game_tree)
}

fn bot_play(
    max_depth: u32,
    times: &mut Vec<u128>,
    bot_player: NonZeroU8,
    mut game_tree: GameNode<Power4>,
) -> GameNode<Power4> {
    let start = std::time::Instant::now();
    game_tree.explore_children(
        bot_player,
        max_depth,
        game_tree.expect_game().plays() as u32,
    );
    //println!("Tree:\n {}", game_tree.debug(3));
    //println!("Into best child...");
    game_tree = game_tree.into_best_child();
    let time = start.elapsed().as_millis();
    times.push(time);
    println!("Done in {}ms", time);
    let weight_opt = game_tree.weight();
    if weight_opt.is_some_and(|it| it > i32::MAX - 1000) {
        println!("You're dead, sorry.");
    } else if weight_opt.is_some_and(|it| it < i32::MIN + 1000) {
        println!("Ok I'm basically dead...");
    }
    debug_assert!(game_tree.game().is_some());
    game_tree
}

fn ask_start() -> bool {
    loop {
        println!("Do you want to start? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Option<char> = input.trim().chars().next();

        if let Some(c) = input {
            if c == 'y' {
                break true;
            } else if c == 'n' {
                break false;
            }
        }
    }
}

fn get_user_input() -> usize {
    loop {
        println!("please specify a column from 1 to 7:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Option<usize> = input.trim().parse().ok();

        if let Some(column) = input {
            if column == 0 || column > 7 {
                println!("Invalid move: {column}\n");
                continue;
            }
            return column;
        }
    }
}
