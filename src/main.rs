use std::io;
use std::num::NonZeroU8;
use crate::game::Game;
use crate::power4::Power4;

mod power4;
mod game;

fn main() {
    let mut game = power4::Power4::new();

    let p1 = NonZeroU8::new(1).unwrap();
    let p2 = NonZeroU8::new(2).unwrap();

    let mut current_player = p1;

    loop {
        game.print();
        println!();
        println!("Player {current_player}'s turn, please specify a column from 1 to 7:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Option<usize> = input.trim().parse().ok();

        if let Some(column) = input {
            let play_result = game.play(current_player, column - 1);

            if let Err(message) = play_result {
                println!("Invalid move: {message}\n");
                continue;
            }
            if game.get_score(current_player) == <Power4 as Game>::Score::MAX {
                println!("Player {current_player} won!");
                break;
            }
            if game.is_full() {
                println!("Draw!");
                break;
            }
        }
        current_player = if current_player == p1 { p2 } else { p1 };
    };
}
