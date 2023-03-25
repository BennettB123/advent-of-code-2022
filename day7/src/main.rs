// Advent of Code 2022
// Day 7

use std::env;
use std::fs;
mod utils;
use utils::{Directory, File, FileSystem};

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

	let mut root_dir = Directory {
		parent: None,
		dirs: vec![],
		files: vec![],
		name: String::from("/"),
	};
	let mut fs = FileSystem::new(&mut root_dir);

	build_file_system(file_contents, &mut fs);

	println!("#############################");
	println!("### Advent of Code, Day 7 ###");
	println!("#############################");
}

fn build_file_system<'a>(_contents: String, fs: &'a mut FileSystem<'a>) {
	let mut dir_a = Directory {
		parent: Some(fs.current_dir),
		dirs: vec![],
		files: vec![],
		name: "dir_a".to_string(),
	};
	
	let mut file_a = File {
		name: "file_a".to_string(),
		size: 55,
	};

	let mut file_b = File {
		name: "file_b".to_string(),
		size: 22,
	};

	

}