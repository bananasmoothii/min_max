use std::io;
use std::num::NonZeroU8;

use crate::bot::Bot;
use crate::game::connect4::Power4;
use crate::game::player::Player;
use crate::game::Game;

mod bot;
mod game;
mod min_max;
mod scalar;

fn main() {
    let max_depth = 9;
    let bot_vs_bot = true;

    let p1 = NonZeroU8::new(1).unwrap();
    let p2 = NonZeroU8::new(2).unwrap();

    let bot_player: NonZeroU8 = p2;

    let mut current_player = if bot_vs_bot || ask_start() { p1 } else { p2 };

    let mut bot: Bot<Power4> = Bot::new(p2, max_depth);

    let mut other_bot: Bot<Power4> = Bot::new(p1, max_depth);

    let mut p1_score: i32 = 0;
    loop {
        println!();
        bot.expect_game().print();
        println!("Scores: {p1_score} for player 1");
        println!();
        println!("Player {current_player}'s turn");
        if current_player == bot_player {
            let play = bot.play();
            if bot_vs_bot {
                other_bot.other_played(play).unwrap();
            }
        } else {
            if bot_vs_bot {
                let play = other_bot.play();
                bot.other_played(play).unwrap();
            } else {
                let result = player_play(&mut bot);
                if let Err(err) = result {
                    println!("Invalid move: {err}\n");
                    continue;
                }
            }
        }

        let game = bot.expect_game();

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
    println!("Average time: {}ms", bot.average_time());
}

fn player_play(bot: &mut Bot<Power4>) -> Result<(), &str> {
    let column = get_user_input();
    let play = column - 1;
    bot.other_played(play)
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
