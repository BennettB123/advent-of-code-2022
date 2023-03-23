// Advent of Code 2022
// Day 4

// Notes:
//  Each line in the input file contains a pair of sections for elves to clean

// Questions:
// 1. how many pairs have a section that fully contains the other section?
// 2. how many pairs have overlapping sections

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

	let section_pairs: Vec<SectionPair> = process_file_contents(file_contents);

	println!("#############################");
	println!("### Advent of Code, Day 4 ###");
	println!("#############################");
	println!("fully contained pairs: {}", count_fully_contained_pairs(&section_pairs));
	println!("overlapping pairs: {}", count_overlapping_pairs(&section_pairs));
}

struct Section {
	start: u32,
	end: u32,
}

impl Section {
	fn parse(input: &str) -> Self {
		let contents = input.split('-').collect::<Vec<&str>>();
		Section {
			start: contents[0].parse::<u32>().unwrap(),
			end: contents[1].parse::<u32>().unwrap(),
		}
	}
}

struct SectionPair {
	section1: Section,
	section2: Section,
}

impl SectionPair {
	fn parse(input: &str) -> Self {
		let contents = input.split(',').collect::<Vec<&str>>();
		SectionPair {
			section1: Section::parse(contents[0]),
			section2: Section::parse(contents[1]),
		}
	}

	fn is_fully_contained(&self) -> bool {
		(self.section1.start >= self.section2.start &&
		self.section1.end <= self.section2.end) ||
		(self.section2.start >= self.section1.start &&
		self.section2.end <= self.section1.end)
	}

	fn is_overlapping(&self) -> bool {
		self.section1.start <= self.section2.end &&
		self.section2.start <= self.section1.end
	}
}

fn process_file_contents(contents: String) -> Vec<SectionPair>{
	let mut pairs: Vec<SectionPair> = vec![];
	for line in contents.lines() {
		pairs.push(SectionPair::parse(line));
	}
	pairs
}

fn count_fully_contained_pairs(pairs: &Vec<SectionPair>) -> u32 {
	let mut total = 0;
	for pair in pairs {
		if pair.is_fully_contained() {
			total += 1;
		}
	}
	total
}

fn count_overlapping_pairs(pairs: &Vec<SectionPair>) -> u32 {
	let mut total = 0;
	for pair in pairs {
		if pair.is_overlapping() {
			total += 1;
		}
	}
	total
}