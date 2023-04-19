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

	for i in 0..cpu.register_history.len() {
		println!("cycle[{}]={}", i, cpu.register_history[i]);
	}

	println!("################################");
	println!("#### Advent of Code, Day 10 ####");
	println!("################################");
	println!("Sum of signal strengths: {}", cpu.get_sum_of_signal_strengths());
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
	cycle_count: u32,
	register_history: Vec<i32>,
}

impl Cpu {
	fn new() -> Self {
		Cpu {
			register: 1,
			cycle_count: 0,
			register_history: vec![1],
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
				self.cycle_count += 1;
				self.register_history.push(self.register);		
            }
            Instruction::Addx(value) => {
				// addx instructions take 2 cycles
				self.cycle_count += 1;
				self.register_history.push(self.register);

				self.cycle_count += 1;
				self.register_history.push(self.register);
				self.register += value;
            }
        }
    }

	fn get_sum_of_signal_strengths(&self) -> i32 {
		(self.register_history[20] * 20) + 
		(self.register_history[60] * 60) + 
		(self.register_history[100] * 100) + 
		(self.register_history[140] * 140) + 
		(self.register_history[180] * 180) + 
		(self.register_history[220] * 220)
	}
}