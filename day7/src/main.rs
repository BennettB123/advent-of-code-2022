// Advent of Code 2022
// Day 7

// Notes:
//	I spent countless hours attempting to solve this puzzle by using a tree-like structure
//  where each directory contains a list of child directories, and also holds a reference to its parent.
//	This led to a headache of borrow-checker errors due to a directory's reference being shared around. 
//	I attempted to circumvent this using Rc and Refcell, but decided it beyond my rust knowledge at this point.
//	
//	I took heavy inspiration from GitHub user gbegerow's solution that can be found here (https://github.com/gbegerow/advent-of-code/blob/main/aoc_2022_07/src/lib.rs)
//	It is a much simpler approach, where every directory is stored in a Vector. Each directory
//	contains a "reference" to its parent via its parent's index in the vector of directories.
//	This allows you to avoid sharing actual references of each directory's parent

use std::env;
use std::fs;

const MAX_DIR_SIZE: usize = 100000;
const TOTAL_DISK_SIZE: usize = 70000000;
const UNUSED_SIZE_REQUIREMENT: usize = 30000000;

pub struct Directory {
	name: String,
	size: usize,
	parent: usize,
}

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

	let file_system = build_file_system(file_contents);
	let answer1: usize = get_answer_1(&file_system);
	let answer2: usize = get_answer_2(&file_system);

	println!("#############################");
	println!("### Advent of Code, Day 7 ###");
	println!("#############################");
	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);
}

fn build_file_system(contents: String) -> Vec<Directory>{
	let root = 0;
	let mut fs: Vec<Directory> = vec![Directory {
		name: "/".to_string(),
		size: 0,
		parent: 0,
	}];
	let mut curr_dir = root;

	for line in contents.lines() {
		let splits: Vec<&str> = line.split(' ').collect();

		// check if the line is a "cd" command
		if splits[0] == "$" {
			if splits[1] == "cd" {
				match splits[2] {
					"/" => curr_dir = root,
					".." => curr_dir = fs[curr_dir].parent,
					dir_name => {
						fs.push(Directory {
							name: dir_name.to_string(),
							size: 0,
							parent: curr_dir,
						});
						curr_dir = fs.len() - 1;
					}
				}
			}
		}
		else if splits[0] == "ls" {
			//  ignore "ls" commands. We can assume every non-command line is output from "ls"
		}
		// this must be output from an ls command from curr_dir
		else {
			match splits[0] {
				"dir" => (),	// don't create new Directory here. Only create them when "cd"-ing into them
				size => {
					let mut parent = curr_dir;
					loop {
						fs[parent].size += size.parse::<usize>().unwrap();
						if parent == root {
							break;
						}
						parent = fs[parent].parent;
					}
				}
			}
		}
	}

	fs
}

fn get_answer_1(file_system: &Vec<Directory>) -> usize {
	let mut total: usize = 0;
	for dir in file_system {
		if dir.size <= MAX_DIR_SIZE {
			total += dir.size;
		}
	}
	total	
}

fn get_answer_2(file_system: &Vec<Directory>) -> usize {
	let curr_disk_usage = file_system[0].size;
	let curr_unused_disk: usize = TOTAL_DISK_SIZE - curr_disk_usage;
	let need_to_delete: usize = UNUSED_SIZE_REQUIREMENT - curr_unused_disk;

	let mut curr_size_to_delete: usize = TOTAL_DISK_SIZE;
	for dir in file_system {
		if dir.size > need_to_delete && dir.size < curr_size_to_delete {
			curr_size_to_delete = dir.size;
		}
	}
	
	curr_size_to_delete
}