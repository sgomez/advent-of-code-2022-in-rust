use std::{env, fs};

use day03::models::game::Game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("File not found");

    let game = Game::build_from_str(&input);

    let sum_of_priorities: u32 = game
        .find_common_items()
        .iter()
        .map(|item| item.priority())
        .sum();

    println!("The solution is {sum_of_priorities}");

    let game = Game::build_from_str(&input);

    let sum_of_commons: u32 = game
        .find_common_items_by_group()
        .iter()
        .map(|item| item.priority())
        .sum();

    println!("The solution is {sum_of_commons}");
}
