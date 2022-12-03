use std::{env, fs};

use day02::models::Game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("File not found");

    let mut game = Game::new();

    input
        .split("\n")
        .for_each(|play| game.play_strategy_one_from_str(play));
    let scores = game.scores();

    println!("The score is: {scores}");

    let mut game = Game::new();
    input
        .split("\n")
        .for_each(|play| game.play_strategy_two_from_str(play));
    let scores = game.scores();

    println!("The score is: {scores}");
}
