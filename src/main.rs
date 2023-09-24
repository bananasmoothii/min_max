use std::io;
use std::num::NonZeroU8;
use crate::game::Game;
use crate::power4::Power4;

mod power4;
mod game;

fn main() {
    let mut game = Power4::new();

    let p1 = NonZeroU8::new(1).unwrap();
    let p2 = NonZeroU8::new(2).unwrap();

    let mut current_player = p1;

    let mut p1_score: i32 = 0;
    loop {
        println!();
        game.print();
        println!("Scores: {p1_score} for player 1");
        println!();
        println!("Player {current_player}'s turn, please specify a column from 1 to 7:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Option<usize> = input.trim().parse().ok();

        if let Some(column) = input {
            if column == 0 {
                println!("Invalid move:\n");
                continue;
            }
            let play_result = game.play(current_player, column - 1);
            p1_score = game.get_score(p1);

            if let Err(message) = play_result {
                println!("Invalid move: {message}\n");
                continue;
            }

            if p1_score == <Power4 as Game>::Score::MAX || p1_score == <Power4 as Game>::Score::MIN {
                println!("Player {current_player} won!");
                break;
            }
            if game.is_full() {
                println!("Draw!");
                break;
            }
            current_player = if current_player == p1 { p2 } else { p1 };
        }
    };
    game.print();
}
