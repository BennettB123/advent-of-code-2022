// Advent of Code 2022
// Day 5

// Notes:
//  The input provided was split up and "massaged" for easier processing.
//	The contents of each stack are located in ./inputs/stacks-input.txt
//	The contents of each move are located in ./inputs/moves-input.txt

// Questions:
// 1. What's on top of each stack after performing all the moves?
// 2. What's on top of each stack after performing all moves if all the crates in a move maintain their order?

use std::env;
use std::fs;

mod stack;
use stack::Stack;
mod mover;
use mover::Move;

fn main() {
	// get file path from commandline input
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		panic!("Provide the input file paths as command line parameters: <path_to_stacks> <path_to_moves>");
	}

	let stacks_file = &args[1];
	let moves_file = &args[2];

	let stacks_file_contents = fs::read_to_string(stacks_file);
	let stacks_file_contents = match stacks_file_contents {
		Ok(contents) => contents,
		Err(_) => panic!("Could not find input file {}", stacks_file),
	};

	let moves_file_contents = fs::read_to_string(moves_file);
	let moves_file_contents = match moves_file_contents {
		Ok(contents) => contents,
		Err(_) => panic!("Could not find input file {}", moves_file),
	};

	let mut stacks: Vec<Stack<char>> = process_stacks_file(stacks_file_contents);
	let moves: Vec<Move> = process_moves_file(moves_file_contents);

	for m in moves {
		perform_move(&mut stacks, m);
	}

	println!("#############################");
	println!("### Advent of Code, Day 5 ###");
	println!("#############################");
	print!("Here's what's on top of each stack: ");
	for stack in stacks {
		match stack.peek() {
			Some(v) => print!("{}", v),
			_ => (),
		}
	}
}

fn process_stacks_file(content: String) -> Vec<Stack<char>> {
	// create stack with empty first element to combat the move input indexing by 1
	let mut stacks: Vec<Stack<char>> = vec![];
	stacks.push(Stack::new());

	for line in content.lines() {
		let mut new_stack: Stack<char> = Stack::new();
		
		// get values from line and push each to the new stack
		let values: Vec<&str> = line.split(' ').collect();
		for v in values {
			new_stack.push(v.chars().next().unwrap());
		}
		stacks.push(new_stack);
	}
	
	stacks
}

fn process_moves_file(content: String) -> Vec<Move> {
	let mut moves: Vec<Move> = vec![];
	for line in content.lines() {
		let values: Vec<&str> = line.split('-').collect();
		moves.push(Move {
			from: values[1].parse::<usize>().unwrap(),
			to: values[2].parse::<usize>().unwrap(),
			amount_to_move: values[0].parse::<u32>().unwrap(),
		})
	}
	moves
}

fn perform_move<T>(stacks: &mut Vec<Stack<T>>, m: Move) {
	let mut vals_to_move: Stack<T> = Stack::new();

	for _ in 0..m.amount_to_move {
		match stacks[m.from].pop() {
			Some(v) => vals_to_move.push(v),
			None => panic!("Oh no! A stack was empty during a pop!"),
		}
	}
	for _ in 0..m.amount_to_move {
		match vals_to_move.pop() {
			Some(v) => stacks[m.to].push(v),
			None => panic!("Oh no! A stack was empty during a pop!"),
		}
	}
}