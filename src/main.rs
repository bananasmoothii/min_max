use std::io;
use std::num::NonZeroU8;

use crate::game::player::Player;
use crate::game::power4::Power4;
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

    let mut p1_score: i32 = 0;
    loop {
        println!();
        game_tree.game.print();
        println!("Scores: {p1_score} for player 1");
        println!();
        println!("Player {current_player}'s turn");
        if current_player == bot_player {
            let start = std::time::Instant::now();
            game_tree.explore_children(bot_player, max_depth, game_tree.game.plays() as u32);
            println!("Tree:\n {}", game_tree.debug(3));
            println!("Into best child...");
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
        } else {
            let column = get_user_input();
            let had_children = !game_tree.children().is_empty();
            let (is_known_move, mut new_game_tree) = game_tree.try_into_child(column - 1);
            if is_known_move {
                game_tree = new_game_tree;
            } else {
                // Here, new_game_tree is actually game_tree, the ownership was given back to us
                if had_children {
                    println!("Unexpected move... Maybe you are a pure genius, or a pure idiot.");
                }
                let result = new_game_tree.game.play(current_player, column - 1);
                if let Err(e) = result {
                    println!("{}", e);
                    game_tree = new_game_tree;
                    continue;
                }
                let depth = new_game_tree.depth() + 1;
                game_tree = GameNode::new_root(new_game_tree.game, current_player.other(), depth);
            }
        }

        p1_score = game_tree.game.get_score(p1);

        if p1_score == <Power4 as Game>::Score::MAX || p1_score == <Power4 as Game>::Score::MIN {
            println!("Player {current_player} won!\n");
            game_tree.game.print();
            break;
        }
        if game_tree.game.is_full() {
            println!("Draw!\n");
            game_tree.game.print();
            break;
        }
        current_player = current_player.other();
    }
    println!(
        "Average time: {}ms",
        times.iter().sum::<u128>() / times.len() as u128
    );
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
