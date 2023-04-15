// Advent of Code 2022
// Day 8

// Notes:
//	This code is quite ugly. I took a very inefficient route to solving these problems.
//	For your own sanity, please don't try to understand what I'm doing. thanks.

use std::{env, fs, cmp};

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

	let forest = parse_input_to_forest(file_contents);
	let num_visible = get_num_visible_trees(&forest);
	let highest_scenic_score = get_highest_scenic_score(&forest);

	println!("#############################");
	println!("### Advent of Code, Day 8 ###");
	println!("#############################");
	println!("Number of visible trees: {}", num_visible);
	println!("Highest scenic score: {}", highest_scenic_score);
}

fn parse_input_to_forest (input: String) -> Vec<Vec<u32>> {
	let mut forest = vec![];

	for line in input.lines() {
		let mut row_of_trees = vec![];
		for tree in line.trim().chars() {
			match tree.to_digit(10) {
				Some(height) => row_of_trees.push(height),
				None => (),
			}
		}
		forest.push(row_of_trees);
	}

	forest
}

fn get_num_visible_trees(forest: &Vec<Vec<u32>>) -> u32 {
	let mut visible_trees = 0;

	for x in 0..forest[0].len() {	// each row
		for y in 0..forest.len() {	// each column
			if is_tree_visible(&forest, x, y) {
				visible_trees += 1;
			}
		}
	}
	visible_trees
}

fn is_tree_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
	is_tree_visible_from_north(forest, x, y) ||
	is_tree_visible_from_east(forest, x, y) ||
	is_tree_visible_from_south(forest, x, y) ||
	is_tree_visible_from_west(forest, x, y)
}

fn is_tree_visible_from_north(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
	let size_of_tree: u32 = forest[y][x];
	let mut num_trees_to_check: i32 = y as i32;
	for row in forest {
		if num_trees_to_check <= 0 {
			break;
		}
		if row[x] >= size_of_tree {
			return false;
		}
		num_trees_to_check -= 1;
	}
	true
}

fn is_tree_visible_from_east(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
	let size_of_tree: u32 = forest[y][x];
	let trees_to_check = &forest[y][x + 1..];
	for tree in trees_to_check.iter().rev() {	// reverse the row, so we can easily search "left to right" instead of "right to left"
		if *tree >= size_of_tree {
			return false;
		}
	}
	true
}

fn is_tree_visible_from_south(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
	let size_of_tree: u32 = forest[y][x];
	let mut num_trees_to_check: i32 = (forest.len() - y - 1) as i32;
	for row in forest.iter().rev() {
		if num_trees_to_check <= 0 {
			break;
		}
		if row[x] >= size_of_tree {
			return false;
		}
		num_trees_to_check -= 1;
	}
	true
}

fn is_tree_visible_from_west(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
	let size_of_tree: u32 = forest[y][x];
	let trees_to_check = &forest[y][..x];
	for tree in trees_to_check {
		if *tree >= size_of_tree {
			return false;
		}
	}
	true
}

fn get_highest_scenic_score(forest: &Vec<Vec<u32>>) -> u32 {
	let mut highest = 0;
	for x in 0..forest[0].len() {	// each row
		for y in 0..forest.len() {	// each column
			highest = cmp::max(highest,
				get_scenic_score_north(forest, x, y) *
				get_scenic_score_east(forest, x, y) *
				get_scenic_score_south(forest, x, y) *
				get_scenic_score_west(forest, x, y)
			)
		}
	}
	highest
}

fn get_scenic_score_north(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
	let size_of_tree: u32 = forest[y][x];
	let num_trees_to_check: usize = y;
	let mut score: u32 = 0;
	for row in 1..=num_trees_to_check {
		if forest[y - row][x] < size_of_tree {
			score += 1
		}
		else {
			score += 1;
			break;
		}
	}
	score
}

fn get_scenic_score_east(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
	let size_of_tree: u32 = forest[y][x];
	let num_trees_to_check = forest[0].len() - x - 1;
	let mut score: u32 = 0;
	for col in 1..=num_trees_to_check {
		if forest[y][x + col] < size_of_tree {
			score += 1
		}
		else {
			score += 1;
			break;
		}
	}
	score
}

fn get_scenic_score_south(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
	let size_of_tree: u32 = forest[y][x];
	let num_trees_to_check: usize = forest.len() - y - 1;
	let mut score: u32 = 0;
	for row in 1..=num_trees_to_check {
		if forest[y + row][x] < size_of_tree {
			score += 1
		}
		else {
			score += 1;
			break;
		}
	}
	score
}

fn get_scenic_score_west(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
	let size_of_tree: u32 = forest[y][x];
	let num_trees_to_check = x;
	let mut score: u32 = 0;
	for col in 1..=num_trees_to_check {
		if forest[y][x - col] < size_of_tree {
			score += 1
		}
		else {
			score += 1;
			break;
		}
	}
	score
}