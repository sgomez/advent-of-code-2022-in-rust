use std::fs;

use day01::models::Expedition;

fn main() {
    let input = fs::read_to_string("data.txt").expect("File not found");
    let expedition = Expedition::build_from_str(&input);
    let solution = expedition.total_calories_from_n_biggest(1);
    println!("Solution first problem: {solution}");
    let solution = expedition.total_calories_from_n_biggest(3);
    println!("Solution second problem: {solution}");
}
