// Advent of Code 2022
// Day 2

// Note: The following code is quite ugly... proceed with caution

// Notes:
//  The elves are playing Rock Paper Scissors.
//  You've been given a cheat sheet for it.
//  The first column is your opponent's choice:	A = Rock, B = Paper, C = Scissors
//  The second column is your choice:  			X = Rock, Y = Paper, Z = Scissors
//	Scoring:
//		Lose = 0, Tie = 3, Win = 6
//		Your choice: Rock = 1, Paper = 2, Scissors = 3

// Questions:
// 1. What is the total score?
// 2. Now the second column is the expected outcome. X = lose, Y = Draw, Z = Win. What is the new total score?

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

	let games = process_file_contents(file_contents);

	println!("#############################");
	println!("### Advent of Code, Day 2 ###");
	println!("#############################");
	println!();
	println!("Total score: {}", calculate_total_points(&games));
}

#[derive(PartialEq)]
#[derive(Debug)]
enum RockPaperScissors {
	Rock,
	Paper,
	Scissors,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GameResult {
	Win,
	Lose,
	Tie,
}

struct Game {
	my_choice: RockPaperScissors,
	their_choice: RockPaperScissors,
	result: GameResult,
	points: u32,
}

fn parse_to_rock_paper_scissors (input: &str) -> RockPaperScissors {
	match input {
		"A" | "X" => RockPaperScissors::Rock,
		"B" | "Y" => RockPaperScissors::Paper,
		_ => RockPaperScissors::Scissors,
	}
}

fn parse_to_game_result(input: &str) -> GameResult {
	match input {
		"X" => GameResult::Lose,
		"Y" => GameResult::Tie,
		_ => GameResult::Win,
	}
}

fn fight(my_choice: &RockPaperScissors, their_choice: &RockPaperScissors) -> (GameResult, u32){
	match *my_choice {
		RockPaperScissors::Rock => {
			let win_bonus: u32 = 1;
			if *their_choice == RockPaperScissors::Rock {
				(GameResult::Tie, 3 + win_bonus)
			}
			else if *their_choice == RockPaperScissors::Paper {
				(GameResult::Lose, 0 + win_bonus)
			}
			else {
				(GameResult::Win, 6 + win_bonus)
			}
		},
		RockPaperScissors::Paper => {
			let win_bonus: u32 = 2;
			if *their_choice == RockPaperScissors::Rock {
				(GameResult::Win, 6 + win_bonus)
			}
			else if *their_choice == RockPaperScissors::Paper {
				(GameResult::Tie, 3 + win_bonus)
			}
			else {
				(GameResult::Lose, 0 + win_bonus)
			}
		},
		RockPaperScissors::Scissors => {
			let win_bonus: u32 = 3;
			if *their_choice == RockPaperScissors::Rock {
				(GameResult::Lose, 0 + win_bonus)
			}
			else if *their_choice == RockPaperScissors::Paper {
				(GameResult::Win, 6 + win_bonus)
			}
			else {
				(GameResult::Tie, 3 + win_bonus)
			}
		}
	}
}

fn get_my_choice_for_expected_result(their_choice: &RockPaperScissors, result: &GameResult) -> RockPaperScissors {
	match *their_choice {
		RockPaperScissors::Rock => {
			if *result == GameResult::Win {
				RockPaperScissors::Paper
			}
			else if *result == GameResult::Lose {
				RockPaperScissors::Scissors
			}
			else {
				RockPaperScissors::Rock
			}
		},
		RockPaperScissors::Paper => {
			if *result == GameResult::Win {
				RockPaperScissors::Scissors
			}
			else if *result == GameResult::Lose {
				RockPaperScissors::Rock
			}
			else {
				RockPaperScissors::Paper
			}
		},
		RockPaperScissors::Scissors => {
			if *result == GameResult::Win {
				RockPaperScissors::Rock
			}
			else if *result == GameResult::Lose {
				RockPaperScissors::Paper
			}
			else {
				RockPaperScissors::Scissors
			}
		}
	}
}

fn process_file_contents(contents: String) -> Vec<Game> {
	let mut games = Vec::<Game>::new();

	for line in contents.lines() {
		let choices = line.split(' ').collect::<Vec<&str>>();
		let their_choice: RockPaperScissors = match choices.get(0) {
			Some(r) => parse_to_rock_paper_scissors(r),
			None => panic!("Your choice was not valid!")
		};
		let required_result: GameResult = match choices.get(1) {
			Some(r) => parse_to_game_result(r),
			None => panic!("Invalid required game result!"),
		};
		let my_choice = get_my_choice_for_expected_result(&their_choice, &required_result);
		let (result, points) = fight(&my_choice, &their_choice);

		games.push(Game {
			my_choice,
			their_choice,
			result,
			points,
		});
	}
	games
}

fn calculate_total_points(games: &Vec<Game>) -> u32 {
	let mut total: u32 = 0;
	for game in games {
		total += game.points;
	}
	total
}