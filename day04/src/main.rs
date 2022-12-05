use std::{env, fs};

use day04::models::game::Game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("File not found");

    let game = Game::from_str(&input);
    let count_contains = game.count_contains();
    let count_overlaps = game.count_overlaps();

    println!("The solution is {count_contains} contained");
    println!("The solution is {count_overlaps} overlaped");
}
