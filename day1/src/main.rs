// Advent of Code 2022
// Day 1

// Notes:
//  The Elves track their calories in input.txt
//  Each line is an amount of calories. The double line breaks separate each Elf

// Questions:
// 1. What is the maximum calories that a single elf is carrying?
// 2. How many calories are the top 3 elves carrying?

use std::env;
use std::fs;

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

	let calories = process_calories(file_contents);

	println!("#############################");
	println!("### Advent of Code, Day 1 ###");
	println!("#############################");
	println!("The elf with the most calories has {}", get_max_calories(&calories).0);
	println!("The top 3 elves have a total of {}", get_top_3_elf_calories(&calories));
}

// converts the file input contents into a Vec<u32> representing the calories of each elf
fn process_calories(calories: String) -> Vec<u32> {
	let mut elf_calories: Vec<u32> = Vec::new();
	let mut current_elf: u32 = 0;
	
	for line in calories.lines(){
		let trimmed_line = line.trim();
		if trimmed_line == "" {
			elf_calories.push(current_elf);
			current_elf = 0;
		}
		else {
			let calorie = trimmed_line.parse::<u32>();
			match calorie {
				Ok(v) => current_elf += v,
				Err(e) => println!("Could not parse line from file: '{}'. Error: {}", trimmed_line, e),
			}
		}
	}

	elf_calories
}

// return type: (max_calories, index)
fn get_max_calories(calories: &Vec<u32>) -> (u32, usize) {
	let mut max: u32 = 0;
	let mut index: usize = 0;

	for (idx, calorie) in calories.iter().enumerate() {
		if *calorie > max {
			max = *calorie;
			index = idx;
		}
	}
	return (max, index);
}

// returns the total calories of the top 3 calorie dense elves
fn get_top_3_elf_calories(calories: &Vec<u32>) -> u32 {
	let mut calories_clone = calories.clone();
	let mut index_to_remove: usize;
	let elf_1: u32;
	let elf_2: u32;
	let elf_3: u32;
	
	(elf_1, index_to_remove) = get_max_calories(&calories_clone);
	calories_clone.remove(index_to_remove);
	(elf_2, index_to_remove) = get_max_calories(&calories_clone);
	calories_clone.remove(index_to_remove);
	(elf_3, _) = get_max_calories(&calories_clone);

	elf_1 + elf_2 + elf_3
}