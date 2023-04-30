// Advent of Code 2022
// Day 14

use std::{collections::VecDeque, env, fs};

fn main() {
    // get file path from commandline input
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Provide the input file's path as a command line parameter");
    }

    let input_file = &args[1];
    let file_contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(err) => panic!("Could not open input file {}. Reason: {}", input_file, err),
    };

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 14 ####");
    println!("################################");
}

fn parse_file(contents: &String) {}
