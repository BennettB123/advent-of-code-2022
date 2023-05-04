// Advent of Code 2022
// Day 14

use std::{
    cmp::{max, min},
    env, fs,
};

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

    // part 1
    let mut cave_part_1 = parse_file_to_cave(&file_contents);
    let mut part_1_num_sand = 0;
    while cave_part_1.add_sand() {
        part_1_num_sand += 1;
    }

    // part 2
    let mut cave_part_2 = parse_file_to_cave(&file_contents);
    let highest_y = cave_part_2.find_highest_rock_y();
    cave_part_2.add_line_of_rocks((0, highest_y + 2), (cave_part_2.width - 1, highest_y + 2));
    let mut part_2_num_sand = 0;
    while cave_part_2.add_sand() {
        part_2_num_sand += 1;
    }

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 14 ####");
    println!("################################");
    println!(
        "Part 1: {} pieces of sand were able to fall and come to rest",
        part_1_num_sand
    );
    println!(
        "Part 2: {} pieces of sand were able to fall and come to rest",
        part_2_num_sand
    );
}

#[derive(Clone, Copy)]
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
    const SAND_START_X: usize = 500;
    const SAND_START_Y: usize = 0;

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

    fn add_material(&mut self, x: usize, y: usize, mat: Material) {
        self.cave[y][x] = mat;
    }

    fn material_at_x_y(&self, x: usize, y: usize) -> Option<&Material> {
        if x > self.width - 1 || y > self.height - 1 {
            return Option::None;
        }
        Option::Some(&self.cave[y][x])
    }

    fn add_line_of_rocks(&mut self, start: (usize, usize), end: (usize, usize)) {
        let start_x: usize = start.0;
        let start_y: usize = start.1;
        let end_x: usize = end.0;
        let end_y: usize = end.1;

        let x_diff = (start_x as i32 - end_x as i32).abs();

        if x_diff != 0 {
            // adding a horizontal line
            for x in min(start_x, end_x)..=max(start_x, end_x) {
                self.add_material(x, start_y, Material::Rock);
            }
        } else {
            // adding a vertical line
            for y in min(start_y, end_y)..=max(start_y, end_y) {
                self.add_material(start_x, y, Material::Rock);
            }
        }
    }

    // adds a piece of sand at (SAND_START_X, SAND_START_Y) and simulates it falling.
    // returns true if the sand came to rest, or false if the sand falls into the endless void
    // Increments Cave::num_sand if the sand came to rest
    // TODO: please refactor this...
    fn add_sand(&mut self) -> bool {
        let mut sand_x = Cave::SAND_START_X;
        let mut sand_y = Cave::SAND_START_Y;

        // early return if sand already exists are starting location
        match self.material_at_x_y(sand_x, sand_y).unwrap() {
            Material::Sand | Material::Rock => return false,
            _ => (),
        }

        while sand_y < self.height {
            match self.material_at_x_y(sand_x, sand_y + 1) {
                None => break,
                Some(mat) => {
                    match mat {
                        // if space below is air, move down one
                        Material::Air => {
                            sand_y += 1;
                        }
                        _ => match self.material_at_x_y(sand_x - 1, sand_y + 1) {
                            None => break,
                            Some(mat) => {
                                match mat {
                                    // if space down+left is air, move there
                                    Material::Air => {
                                        sand_x -= 1;
                                        sand_y += 1;
                                    }
                                    _ => match self.material_at_x_y(sand_x + 1, sand_y + 1) {
                                        None => break,
                                        Some(mat) => {
                                            match mat {
                                                // if space down+right is air, move there
                                                Material::Air => {
                                                    sand_x += 1;
                                                    sand_y += 1;
                                                }
                                                _ => {
                                                    // if we get here, the sand couldn't move down, left, or right. it has come to rest
                                                    self.add_material(
                                                        sand_x,
                                                        sand_y,
                                                        Material::Sand,
                                                    );
                                                    return true;
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        },
                    }
                }
            }
        }
        false
    }

    fn find_highest_rock_y(&self) -> usize {
        let mut highest = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                match self.material_at_x_y(x, y).unwrap() {
                    Material::Rock => highest = y,
                    _ => (),
                }
            }
        }

        highest
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if x == Cave::SAND_START_X && y == Cave::SAND_START_Y {
                    s.push('X');
                } else {
                    s += &format!("{}", self.cave[y][x]);
                }
            }
            s += "\n";
        }

        println!("{}", s);
    }
}

fn parse_file_to_cave(contents: &String) -> Cave {
    let (highest_x, highest_y) = find_lows_and_highs(contents);
    let mut cave = Cave::new(highest_x + 750, highest_y + 5); // adding a lot of padding so sand doesn't fall outside of the vector's bounds

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
