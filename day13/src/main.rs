// Advent of Code 2022
// Day 13

use std::{env, fs};

enum Packet {
    Empty,
    Value(i32),
    List(Vec<Packet>),
}

struct PairOfPackets {
    p1: Packet,
    p2: Packet,
}

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

    println!("################################");
    println!("#### Advent of Code, Day 13 ####");
    println!("################################");
}

fn parse_file(contents: &String) -> i32 {
    0
}
