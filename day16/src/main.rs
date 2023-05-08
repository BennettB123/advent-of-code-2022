// Advent of Code 2022
// Day 16

use regex::Regex;
use std::{env, fs};

const TOTAL_MINUTES: i32 = 30;

fn main() {
    // get file path from commandline input
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Provide the input file's path as a command line parameter");
    }

    let input_file = &args[1];
    let file_contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(err) => panic!("Could not open input file {}. Reason: {}", input_file, err),
    };

    let valves = parse_input(&file_contents);

    for v in valves {
        println!("{:?}", v);
        println!();
    }

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 16 ####");
    println!("################################");
}

fn parse_input(contents: &String) -> Vec<Valve> {
    let mut valves: Vec<Valve> = vec![];

    // capture 1(Valve name), 2(Flow rate), 3(comma-separated list of tunnels)
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
        .unwrap();

    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let captures: Vec<&str> = re
            .captures(line)
            .unwrap()
            .iter()
            .map(|s| s.unwrap().as_str())
            .collect();

        // create vector of tunnels for this valve
        let mut tunnels = vec![];
        for t in captures[3].split(',') {
            tunnels.push(t.trim());
        }

        valves.push(Valve {
            name: captures[1].parse().unwrap(),
            flow_rate: captures[2].parse().unwrap(),
            tunnels,
            open: false,
        });
    }

    valves
}

fn get_valve_by_name<'a>(valves: &'a Vec<Valve>, name: &str) -> &'a Valve<'a> {
    for v in valves {
        if v.name == name {
            return v
        }
    }
    panic!("Did not find valve with name '{}'!", name);
}

#[derive(Debug)]
struct Valve<'a> {
    name: String,
    flow_rate: i32,
    tunnels: Vec<&'a str>,
    open: bool,
}
