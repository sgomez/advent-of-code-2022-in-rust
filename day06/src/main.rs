use std::fs;

use day06::models::Datastream;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("File not found");

    let datastream = Datastream::from_str(input.as_str(), 4);
    let index = datastream.start_of_message();

    println!("The solution is {index}");

    let datastream = Datastream::from_str(input.as_str(), 14);
    let index = datastream.start_of_message();

    println!("The solution is {index}");
}
