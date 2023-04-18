// Advent of Code 2022
// Day 9

use std::{env, fs};
mod utils;

fn main() {
	// get file path from commandline input
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		panic!("Provide the input file's path as a command line parameter");
	}

	let input_file = &args[1];

	let file_contents = fs::read_to_string(input_file);
	let file_contents = match file_contents {
		Ok(contents) => contents,
		Err(_) => panic!("Could not find input file {}", input_file),
	};

	let all_moves = parse_file_contents(file_contents);

	let mut two_knot_rope = utils::Rope::new(2);
	let mut ten_knot_rope = utils::Rope::new(10);

	for m in all_moves {
		two_knot_rope.move_head(&m);
		ten_knot_rope.move_head(&m);
	}
	
	println!("#############################");
	println!("### Advent of Code, Day 9 ###");
	println!("#############################");
	println!("Unique locations for a 2 knot rope: {}", two_knot_rope.get_unique_tail_locations());
	println!("Unique locations for a 10 knot rope: {}", ten_knot_rope.get_unique_tail_locations());
}

fn parse_file_contents(contents: String) -> Vec<utils::Movement> {
	let mut moves: Vec<utils::Movement> = vec![];
	for line in contents.lines() {
		let dir_and_amount: Vec<&str> = line.split(' ').collect();
		let dir = utils::Direction::parse(dir_and_amount[0]);
		let amount = dir_and_amount[1].parse::<i32>();

		// if we were able to parse the amount, create the Movement
		match amount {
			Ok(amt) => moves.push(utils::Movement::new(dir, amt)),
			Err(_) => panic!("Could not parse movement amount, '{}'.", dir_and_amount[1])
		}
	}

	moves
}