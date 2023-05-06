// Advent of Code 2022
// Day 15

// Notes:
//	This program is not optimized. Both parts of the puzzle are pretty slow, I assume because I'm storing many items in a HashSet
//  I could have took the time to solve the puzzles without HashSets, but I already spent too much time on this day, so I decided to move on.

use regex::Regex;
use std::{collections::HashSet, env, fmt::Display, fs};

const PART_1_Y: i32 = 2000000;
const MIN_DISTRESS_COORDINATE: i32 = 0;
const MAX_DISTRESS_COORDINATE: i32 = 4000000;
const TUNING_PARAMETER_MULTIPLIER: i64 = 4000000;

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

    // part 1
    let mut non_beacon_coords = HashSet::<(i32, i32)>::new();
    for s in &sensors {
        non_beacon_coords.extend(s.get_non_beacon_coords_at_y(PART_1_Y));
    }

    for s in &sensors {
        if non_beacon_coords.contains(&(s.closest_beacon.x, s.closest_beacon.y)) {
            non_beacon_coords.remove(&(s.closest_beacon.x, s.closest_beacon.y));
        }
    }

    // part 2
    // go through each sensor's perimeter coordinates
    // check if any of those coordinates are outside of every other sensor's radius
    let mut distress_beacon = Beacon { x: 0, y: 0 };
    let mut found = false;
    for s1 in &sensors {
        if found {
            break;
        }
        for coord in s1.get_coords_at_sensors_radius() {
            // skip this coordinate if it is outside the limit
            if coord.0 > MAX_DISTRESS_COORDINATE
                || coord.1 > MAX_DISTRESS_COORDINATE
                || coord.0 < MIN_DISTRESS_COORDINATE
                || coord.1 < MIN_DISTRESS_COORDINATE
            {
                continue;
            }

            let mut inside_a_range = false;
            for s2 in &sensors {
                if s2.is_x_y_in_sensors_radius(coord.0, coord.1) {
                    inside_a_range = true;
                }
            }

            if !inside_a_range {
                found = true;
                distress_beacon = Beacon {
                    x: coord.0,
                    y: coord.1,
                }
            }
        }
    }

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 15 ####");
    println!("################################");
    println!(
        "Number of non-beacon positions at y={}: {}",
        PART_1_Y,
        non_beacon_coords.len()
    );
    println!("Location of distress signal: {}", distress_beacon);
    println!(
        "Distress signal's tuning frequency: {}",
        distress_beacon.calculate_tuning_frequency()
    );
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
            "Sensor at ({},{}). Closest Beacon at ({},{})",
            self.x, self.y, self.closest_beacon.x, self.closest_beacon.y
        )
    }
}

impl Sensor {
    fn get_distance_to_beacon(&self) -> i32 {
        (self.x - self.closest_beacon.x).abs() + (self.y - self.closest_beacon.y).abs()
    }

    fn get_non_beacon_coords_at_y(&self, y: i32) -> HashSet<(i32, i32)> {
        let mut coords = HashSet::<(i32, i32)>::new();
        let distance = self.get_distance_to_beacon();
        let distance_from_y = (y - self.y).abs();

        if distance >= distance_from_y {
            for x in self.x - (distance - distance_from_y)..=self.x + (distance - distance_from_y) {
                coords.insert((x, y));
            }
        }

        coords
    }

    fn get_coords_at_sensors_radius(&self) -> HashSet<(i32, i32)> {
        let mut coords = HashSet::<(i32, i32)>::new();
        let mut x: i32;
        let mut y: i32;

        let radius: i32 = self.get_distance_to_beacon() + 1;

        // We start in the upper point (x , y + radius)
        x = self.x;
        y = self.y + radius;

        while x <= self.x + radius && y >= self.y {
            coords.insert((x, y));
            x += 1;
            y -= 1;
        }
        x -= 1;
        y += 1;

        // Going left and down: from (x , y + radius) to (x , y - radius)
        while x >= self.x && y >= self.y - radius {
            coords.insert((x, y));
            x -= 1;
            y -= 1;
        }
        x += 1;
        y += 1;

        // Going left and up: from (x , y - radius) to (x - radius , y)
        while x >= self.x - radius && y <= self.y {
            coords.insert((x, y));
            x -= 1;
            y += 1;
        }
        x += 1;
        y -= 1;

        // Going right and up: from (x - radius , y) to (x , y + radius).
        while x <= self.x && y <= self.y + radius {
            coords.insert((x, y));
            x += 1;
            y += 1;
        }

        coords
    }

    fn is_x_y_in_sensors_radius(&self, x: i32, y: i32) -> bool {
        ((self.x - x).abs() + (self.y - y).abs()) <= self.get_distance_to_beacon()
    }
}

struct Beacon {
    x: i32,
    y: i32,
}

impl Display for Beacon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Beacon {
    fn calculate_tuning_frequency(&self) -> i64 {
        (self.x as i64 * TUNING_PARAMETER_MULTIPLIER) + self.y as i64
    }
}
