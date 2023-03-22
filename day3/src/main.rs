// Advent of Code 2022
// Day 3

// Notes:
//  The elves are packing rucksacks.
//	Each line in the input file is a rucksack broken exactly in half into 2 compartments
//	Each rucksack has a duplicate item that appears in both compartments
//	The "value" of each item is: a-z = 1-26 and A-Z = 27-52

// Questions:
// 1. What is the total value of all duplicate items in the rucksacks
// 2. Every group of 3 lines in the input file is a group of elves. There is 1 common item in each group.
// 	  What is the total value of the common items throughout all the groups.

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

	let rucksacks = process_file_contents(file_contents);

	println!("#############################");
	println!("### Advent of Code, Day 3 ###");
	println!("#############################");
	println!("total duplicates: {}", get_total_duplicates(&rucksacks));
	println!("total badge values: {}", get_total_badge_values(&rucksacks));
}

struct Rucksack {
	compartment_1: String,
	compartment_2: String,
}

impl Rucksack {
	fn find_duplicate(&self) -> char {
		let mut dupe: char = '\0';
		for c1 in self.compartment_1.chars() {
			for c2 in self.compartment_2.chars() {
				if c1 == c2 {
					dupe = c1;
				}
			}
		}
		dupe
	}

	fn get_contents(&self) -> String {
		self.compartment_1.clone() + &self.compartment_2
	}
}

fn process_file_contents(contents: String) -> Vec<Rucksack> {
	let mut rucksacks = Vec::<Rucksack>::new();
	for line in contents.lines() {
		let rucksack_length = line.len();
		let (first_half, last_half) = line.split_at(rucksack_length/2);
		rucksacks.push(Rucksack{
			compartment_1: String::from(first_half),
			compartment_2: String::from(last_half),
		});
	}
	rucksacks
}

fn get_total_duplicates(rucksacks: &Vec<Rucksack>) -> u32 {
	let mut total: u32 = 0;
	for sack in rucksacks {
		total += get_item_value(sack.find_duplicate());
	}
	total
}

fn get_total_badge_values(rucksacks: &Vec<Rucksack>) -> u32 {
	let mut total: u32 = 0;
	let num_groups = rucksacks.len()/3;
	for i in 0..num_groups {
		let dupe = find_duplicate_between_sacks(
			&rucksacks[(i * 3) + 0],
			&rucksacks[(i * 3) + 1],
			&rucksacks[(i * 3) + 2]
		);
		total += get_item_value(dupe);
	}

	total
}

fn find_duplicate_between_sacks(s1: &Rucksack, s2: &Rucksack, s3: &Rucksack) -> char {
	let mut dupe: char = '\0';
	for c1 in s1.get_contents().chars() {
		for c2 in s2.get_contents().chars() {
			for c3 in s3.get_contents().chars() {
				if c1 == c2 && c2 == c3 {
					dupe = c1;
				}
			}
		}
	}
	dupe
}

fn get_item_value(c: char) -> u32 {
	// calculate values of lowercase characters
	if c as u32 >= 97 {
		c as u32 - 96
	}
	else {
		c as u32 - 38
	}
}