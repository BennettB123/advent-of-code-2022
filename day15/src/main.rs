// Advent of Code 2022
// Day 15

use regex::Regex;
use std::{env, fmt::Display, fs};

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

    let sensors = parse_input(&file_contents);

    for s in sensors {
        println!("{}", s);
    }

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 15 ####");
    println!("################################");
}

fn parse_input(contents: &String) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = vec![];

    let re = Regex::new(
        r"(?i)^sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
    )
    .unwrap();

    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let captures = re.captures(line).unwrap();
        sensors.push(Sensor {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
            closest_beacon: Beacon {
                x: captures[3].parse().unwrap(),
                y: captures[4].parse().unwrap(),
            },
        });
    }

    sensors
}

struct Sensor {
    x: i32,
    y: i32,
    closest_beacon: Beacon,
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sensor at ({},{}). Closet Beacon at ({},{})",
            self.x, self.y, self.closest_beacon.x, self.closest_beacon.y
        )
    }
}

struct Beacon {
    x: i32,
    y: i32,
}
