# MinMax algorithm implementation in Rust

## Description

The MinMax algorithm is a recursive algorithm that is used to choose an optimal move for a player assuming that the
opponent is also playing optimally. It is used in two player games such as Tic-Tac-Toe, Connect Four, etc., where
nothing is hidden from either player, and no chance is involved in the game.

## Implementation

This implementation uses multithreading and alpha-beta pruning (removing branches of the game tree that probably are not
going to be chosen) to speed up the algorithm.

## Usage

This game currently plays in the terminal. To play, run the following command (assuming you have Rust installed):

```bash
cargo run --release
```

By default, the bot computes 10 moves ahead. You can change this in the first line of the `main` function
in `src/main.rs`.