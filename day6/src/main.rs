// Advent of Code 2022
// Day 6

use std::env;
use std::fs;

fn main() {
	// get file path from commandline input
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		panic!("Provide the input file path as a command line parameter.");
	}

	let stacks_file = &args[1];

	let file_contents = fs::read_to_string(stacks_file);
	let file_contents = match file_contents {
		Ok(contents) => contents,
		Err(_) => panic!("Could not find input file {}", stacks_file),
	};
	let message: Vec<char> = file_contents.chars().collect();

	// start at 14th element
	let mut i = 13;
	let length = file_contents.len();
	loop {
		if i >= length {
			break;
		}

		// I got lazy and didn't want to create a better algorithm...
		let mut last_four: Vec<char> = vec![
			message[i-13], message[i-12], message[i-11], message[i-10],
			message[i-9], message[i-8], message[i-7], message[i-6],
			message[i-5], message[i-4], message[i-3], message[i-2],
			message[i-1], message[i-0]];
		last_four.sort();
		last_four.dedup();

		if last_four.len() == 14 {
			break;
		}
		i += 1;
	}

	let marker_start = i + 1;

	println!("#############################");
	println!("### Advent of Code, Day 6 ###");
	println!("#############################");
	println!("start-of-message marker found after {} characters", marker_start);
}

