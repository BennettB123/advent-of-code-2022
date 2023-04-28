// Advent of Code 2022
// Day 12

use std::{collections::VecDeque, env, fs};

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

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

    let mut height_map = parse_file_to_height_map(file_contents);
	height_map.build_distance_map();

    println!("################################");
    println!("#### Advent of Code, Day 12 ####");
    println!("################################");
    println!("quickest path is {} moves", height_map.distance_to_end());
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
    distances: Vec<Vec<i32>>,
    checked_locations: VecDeque<(usize, usize)>,
}

impl HeightMap {
    fn new(map: Vec<Vec<u8>>, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Self {
        // build distances map with every entry having -1
        let mut distances = vec![];
        for _ in 0..map.len() {
            let mut distance_row: Vec<i32> = vec![];
            for _ in 0..map[0].len() {
                distance_row.push(-1);
            }
            distances.push(distance_row);
        }

        let mut ret = HeightMap {
            map,
            start_x,
            start_y,
            end_x,
            end_y,
            distances: distances,
            checked_locations: VecDeque::new(),
        };
        ret.set_distance(start_x, start_y, 0);
        ret
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        match self.map.first() {
            Some(row) => row.len(),
            None => 0,
        }
    }

    fn elevation_at_x_y(&self, x: usize, y: usize) -> u8 {
        self.map[y][x]
    }

    fn distance_at_x_y(&self, x: usize, y: usize) -> i32 {
        self.distances[y][x]
    }

	fn distance_to_end(&self) -> i32 {
		self.distance_at_x_y(self.end_x, self.end_y)
	}

    fn set_distance(&mut self, x: usize, y: usize, distance: i32) {
        self.distances[y][x] = distance;
    }

    fn find_eligible_neighbors(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
		let mut new_neighbors = vec![];
        // up
        if let Some((new_x, new_y)) = self.get_neighbor_up(x, y) {
            if !self.have_visited(new_x, new_y) && !self.checked_locations.contains(&(new_x, new_y))
            {
                self.checked_locations.push_back((new_x, new_y));
				new_neighbors.push((new_x, new_y));
            }
        }

        // down
        if let Some((new_x, new_y)) = self.get_neighbor_down(x, y) {
            if !self.have_visited(new_x, new_y) && !self.checked_locations.contains(&(new_x, new_y))
            {
                self.checked_locations.push_back((new_x, new_y));
				new_neighbors.push((new_x, new_y));
            }
        }

        // left
        if let Some((new_x, new_y)) = self.get_neighbor_left(x, y) {
            if !self.have_visited(new_x, new_y) && !self.checked_locations.contains(&(new_x, new_y))
            {
                self.checked_locations.push_back((new_x, new_y));
				new_neighbors.push((new_x, new_y));
            }
        }

        // right
        if let Some((new_x, new_y)) = self.get_neighbor_right(x, y) {
            if !self.have_visited(new_x, new_y) && !self.checked_locations.contains(&(new_x, new_y))
            {
                self.checked_locations.push_back((new_x, new_y));
				new_neighbors.push((new_x, new_y));
            }
        }

		new_neighbors
    }

    fn have_visited(&self, x: usize, y: usize) -> bool {
        self.distance_at_x_y(x, y) >= 0
    }

    fn get_neighbor_up(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y == 0 {
            return Option::None;
        } else if self.elevation_at_x_y(x, y - 1) <= self.elevation_at_x_y(x, y) + 1 {
            return Option::Some((x, y - 1));
        } else {
            return Option::None;
        }
    }

    fn get_neighbor_down(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y == self.height() - 1 {
            return Option::None;
        } else if self.elevation_at_x_y(x, y + 1) <= self.elevation_at_x_y(x, y) + 1 {
            return Option::Some((x, y + 1));
        } else {
            return Option::None;
        }
    }

    fn get_neighbor_left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x == 0 {
            return Option::None;
        } else if self.elevation_at_x_y(x - 1, y) <= self.elevation_at_x_y(x, y) + 1 {
            return Option::Some((x - 1, y));
        } else {
            return Option::None;
        }
    }

    fn get_neighbor_right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x == self.width() - 1 {
            return Option::None;
        } else if self.elevation_at_x_y(x + 1, y) <= self.elevation_at_x_y(x, y) + 1 {
            return Option::Some((x + 1, y));
        } else {
            return Option::None;
        }
    }

    fn build_distance_map(&mut self) {
        self.explore(self.start_x, self.start_y, 0);
    }

	// TODO: THIS IS DEPTH FIRST SEARCH, NOT BREADTH FIRST!!!!
    fn explore(&mut self, x: usize, y: usize, curr_dist: i32) {
		println!("Exploring ({},{}). Distance {}", x, y, curr_dist);
		self.set_distance(x, y, curr_dist);
		
        for n in self.find_eligible_neighbors(x, y) {
			self.explore(n.0, n.1, curr_dist + 1);
		}
    }

    fn height_map_to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if x == self.start_x && y == self.start_y {
                    s.push(START_CHAR);
                } else if x == self.end_x && y == self.end_y {
                    s.push(END_CHAR);
                } else {
                    s.push(parse_height_to_char(self.elevation_at_x_y(x, y)));
                }
            }
            s += "\n";
        }
        s
    }

    fn distance_map_to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if x == self.start_x && y == self.start_y {
                    s += &format!("{: >5}", START_CHAR);
                } else if x == self.end_x && y == self.end_y {
                    s += &format!("{: >5}", END_CHAR);
                } else {
                    s += &format!("{: >5}", &self.distance_at_x_y(x, y))
                }
            }
            s += "\n";
        }
        s
    }
}
