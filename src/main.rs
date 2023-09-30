use std::io;
use std::num::NonZeroU8;
use crate::game::Game;
use crate::game::player::Player;
use crate::game::state::GameState::PlayersTurn;
use crate::min_max::node::GameNode;
use crate::power4::Power4;

mod power4;
mod game;
mod min_max;
mod scalar;

fn main() {
    let p1 = NonZeroU8::new(1).unwrap();
    let p2 = NonZeroU8::new(2).unwrap();

    let mut current_player = p1;

    let bot_player: NonZeroU8 = if ask_start() { p2 } else { p1 };

    let mut original_game_tree: GameNode<Power4> = GameNode::new_root(Power4::new(), p1);
    let mut game_node = &mut original_game_tree;

    let mut p1_score: i32 = 0;
    loop {
        println!();
        game_node.game.print();
        println!("Scores: {p1_score} for player 1");
        println!();
        println!("Player {current_player}'s turn");
        if current_player == bot_player {
            game_node = game_node.choose_best_child_mut().unwrap_or_else(|| panic!("No options to choose from!"));
        } else {
            let column = get_input();
            if let Some(new_node) = game_node.get_node_with_play_mut(column) {
                game_node = new_node; // no need to play, play is already done
            } else {
                println!("Unpredicted move");
                let result = game_node.game.clone().play(current_player, column);
                if let Err(e) = result {
                    println!("Invalid move: {e}\n");
                    continue;
                }
                let new_node = GameNode::new(
                    game_node.game.clone(),
                    game_node.depth() + 1,
                    None,
                    PlayersTurn(current_player.other())
                );
                game_node.children_mut().push(new_node); // add it to tree, we can't own it because we need its reference
                game_node = game_node.children_mut().last_mut().unwrap();
            }
        }

        p1_score = game_node.game.get_score(p1);

        if p1_score == <Power4 as Game>::Score::MAX || p1_score == <Power4 as Game>::Score::MIN {
            println!("Player {current_player} won!");
            game_node.game.print();
            break;
        }
        if game_node.game.is_full() {
            println!("Draw!");
            game_node.game.print();
            break;
        }
        current_player = if current_player == p1 { p2 } else { p1 };
    };
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

fn get_input() -> usize {
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
