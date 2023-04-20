// Advent of Code 2022
// Day 10

use std::{env, fs};

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

	let instruction_set = parse_file_contents(file_contents);
	let mut cpu = Cpu::new();
	cpu.run(&instruction_set);

	let mut crt_screen = CrtScreen::new();
	crt_screen.populate_screen(&cpu.register_history);

	println!("################################");
	println!("#### Advent of Code, Day 10 ####");
	println!("################################");
	println!("Sum of signal strengths: {}", cpu.get_sum_of_signal_strengths());
	println!("CRT Monitor:\n");
	println!("{}", crt_screen.to_string());
}

fn parse_file_contents(contents: String) -> Vec<Instruction> {
	let mut instruction_set = vec![];
	for line in contents.lines() {
		instruction_set.push(Instruction::parse(line));
	}
	instruction_set
}

#[derive(Debug, Clone)]
enum Instruction {
	Noop,
	Addx(i32),
}

impl Instruction {
	fn parse(input: &str) -> Self {
		let split = input.split(' ').collect::<Vec<&str>>();
		match split[0] {
			"noop" => Instruction::Noop,
			"addx" => Instruction::Addx(split[1].parse().unwrap()),
			_ => panic!("Bad instruction found: '{}'", split[0]),
		}
	}
}

struct Cpu {
	register: i32,
	register_history: Vec<i32>,
}

impl Cpu {
	fn new() -> Self {
		Cpu {
			register: 1,
			register_history: vec![],
		}
	}

	fn run(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
				self.register_history.push(self.register);		
            }
            Instruction::Addx(value) => {
				// addx instructions take 2 cycles
				self.register_history.push(self.register);
				self.register_history.push(self.register);
				self.register += value;
            }
        }
    }

	fn get_sum_of_signal_strengths(&self) -> i32 {
		(self.register_history[19] * 20) + 
		(self.register_history[59] * 60) + 
		(self.register_history[99] * 100) + 
		(self.register_history[139] * 140) + 
		(self.register_history[179] * 180) + 
		(self.register_history[219] * 220)
	}
}


const CRT_LIT: char = '#';
const CRT_DIM: char = '.';
const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;
const CRT_TOTAL_PIXELS: usize = CRT_WIDTH * CRT_HEIGHT;

struct CrtScreen {
	display: Vec<char>,
}

impl CrtScreen {
	fn new() -> Self {
		CrtScreen {
			display: vec![],
		}
	}

	fn populate_screen(&mut self, register_history: &Vec<i32>) {
		for cycle in 0..CRT_TOTAL_PIXELS {
			let pos_in_row = (cycle % CRT_WIDTH) as i32;

			if ((register_history[cycle] - 1)..=(register_history[cycle] + 1)).contains(&pos_in_row){
				self.display.push(CRT_LIT);
			}
			else {
				self.display.push(CRT_DIM);
			}
		}
	}

	fn to_string(&self) -> String {
		if self.display.len() < CRT_TOTAL_PIXELS {
			panic!("Screen has not been fully populated yet. Only {} pixels exist, expected {}", self.display.len(), CRT_TOTAL_PIXELS);
		}

		let mut screen = String::new();
		let mut curr_pixel = 1;

		for pixel in &self.display {
			screen.push(*pixel);

			if curr_pixel % CRT_WIDTH == 0 {
				screen.push('\n');
			}
			
			curr_pixel += 1;
		}
		screen
	}
}
