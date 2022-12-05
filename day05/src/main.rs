use day05::models::game::Game;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("File not found");

    let mut game = Game::from_str(&input);
    game.run_procedures();

    let result = game.get_top_stacks();

    println!("The solution is {result} ");

    let mut game = Game::from_str(&input);
    game.run_procedures_in_block();

    let result = game.get_top_stacks();

    println!("The solution is {result} ");
}
