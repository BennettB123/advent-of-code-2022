// Advent of Code 2022
// Day 14

use std::{env, fs};

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

    let mut cave = parse_file_to_cave(&file_contents);

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 14 ####");
    println!("################################");
}

enum Material {
    Air,
    Rock,
    Sand,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Material::Air => write!(f, "."),
            Material::Rock => write!(f, "#"),
            Material::Sand => write!(f, "o"),
        }
    }
}

struct Cave {
    cave: Vec<Vec<Material>>,
    width: usize,
    height: usize,
}

impl Cave {
    fn new(width: usize, height: usize) -> Self {
        let mut cave: Vec<Vec<Material>> = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Material::Air);
            }
            cave.push(row);
        }
        Cave {
            cave,
            width,
            height,
        }
    }

    fn add_line_of_rocks(&mut self, start: (usize, usize), end: (usize, usize)) {
        // 487,45 -> 487,38
        // 487,45 -> 489,45
        let start_x: i32 = start.0 as i32;
        let start_y: i32 = start.1 as i32;
        let end_x: i32 = end.0 as i32;
        let end_y: i32 = end.1 as i32;

        let x_diff = (start_x - end_x).abs();
        let y_diff = (start_y - end_y).abs();
    }

    fn print(&self) {
        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                s += &format!("{}", self.cave[y][x]);
            }
            s += "\n";
        }

        println!("{}", s);
    }
}

fn parse_file_to_cave(contents: &String) -> Cave {
    let (highest_x, highest_y) = find_lows_and_highs(contents);
    println!("highest x = {}, highest y = {}", highest_x, highest_y);
    let mut cave = Cave::new(highest_x, highest_y);

    // parse input string
    for line in contents.lines() {
        let coords_strings = line.split("->").map(|c| c.trim());
        let mut coords: Vec<(usize, usize)> = vec![];
        for s in coords_strings {
            let x: usize = s.split(',').nth(0).unwrap().parse().unwrap();
            let y: usize = s.split(',').nth(1).unwrap().parse().unwrap();
            coords.push((x, y));
        }
        for i in 0..coords.len() - 1 {
            cave.add_line_of_rocks(coords[i], coords[i + 1]);
        }
    }

    cave
}

// Since the input contains coordinates in the range around ~500,
//  this fn finds the highest x and y values, so a 2d Vec can be
//  created with the correct capacity. Return value is (highest_x, highest_y)
fn find_lows_and_highs(contents: &String) -> (usize, usize) {
    let mut highest_x = usize::MIN;
    let mut highest_y = usize::MIN;

    let coords: Vec<&str> = contents
        .split(['\n', ' ', '-', '>'].as_ref())
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .collect();

    for c in coords {
        let x: usize = c.split(',').nth(0).unwrap().parse().unwrap();
        let y: usize = c.split(',').nth(1).unwrap().parse().unwrap();
        if x > highest_x {
            highest_x = x;
        }
        if y > highest_y {
            highest_y = y;
        }
    }

    (highest_x, highest_y)
}
