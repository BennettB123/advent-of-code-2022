// Advent of Code 2022
// Day 12
//
// Notes: To make the problem easier, the program will convert the heightmap to numbers (a=1, z=26).

use std::{env, fs};

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';
const CURRENT_CHAR: char = '@';

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
		Err(err) => panic!("Could not open input file {}. Reason: {}", input_file, err),
	};

	let height_map = parse_file_to_height_map(file_contents);


	println!("################################");
	println!("#### Advent of Code, Day 12 ####");
	println!("################################");
	println!();
	println!("{}", height_map.to_string());
}


fn parse_file_to_height_map(contents: String) -> HeightMap {
	let mut map: Vec<Vec<u8>> = vec![];
	let mut start_x = 0;
	let mut start_y = 0;
	let mut end_x = 0;
	let mut end_y = 0;

	let mut y = 0;
	for line in contents.lines() {
		let mut x = 0;
		let mut row: Vec<u8> = vec![];
		for c in line.chars() {
			if c == START_CHAR {
				start_x = x;
				start_y = y;
			}
			if c == END_CHAR {
				end_x = x;
				end_y = y;
			}
			row.push(parse_char_to_height(c));
			x += 1;
		}
		map.push(row);
		y += 1;
	}

	HeightMap::new(map, start_x, start_y, end_x, end_y)
}

// parses letters to numbers, a=1, b=2, ..., y=25, z=26
// Start 'S' is equal to 'a' and End 'E' is equal to 'z'
// panics if not converted correctly (invalid char)
fn parse_char_to_height(c: char) -> u8 {
	let height: u8 = match c {
		'S' => 1,
		'E' => 26,
		_ => c as u8 - 96,
	};

	if height <= 0 || height >= 27 {
		panic!("encountered an invalid char: '{}'.", c);
	}

	height
}

fn parse_height_to_char(h: u8) -> char {
	(h + 96) as char
}

struct HeightMap {
	map: Vec<Vec<u8>>,
	start_x: usize,
	start_y: usize,
	end_x: usize,
	end_y: usize,
	current_x: usize,
	current_y: usize,
	num_moves: u32,
}

impl HeightMap {
	fn new(map: Vec<Vec<u8>>, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Self {
		HeightMap {
			map,
			start_x,
			start_y,
			end_x,
			end_y,
			current_x: start_x,
			current_y: start_y,
			num_moves: 0,
		}
	}

	fn get_height(&self) -> usize {
		self.map.len()
	}

	fn get_width(&self) -> usize {
		match self.map.first() {
			Some(row) => row.len(),
			None => 0,
		}
	}

	fn get_x_y(&self, x: usize, y: usize) -> u8 {
		self.map[y][x]
	}

	fn to_string(&self) -> String {
		let mut s = String::new();
		for y in 0..self.get_height() {
			for x in 0..self.get_width() {
				if x == self.current_x && y == self.current_y {
					s.push(CURRENT_CHAR);
				}
				else if x == self.start_x && y == self.start_y {
					s.push(START_CHAR);
				}
				else if x == self.end_x && y == self.end_y {
					s.push(END_CHAR);
				}
				else {
					s.push(parse_height_to_char(self.get_x_y(x, y)));
				}
			}
			s += "\n";
		}
		s
	}
}
