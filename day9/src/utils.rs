use std::fmt;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn parse(s: &str) -> Self {
		match s {
			"U"|"u" => Direction::Up,
			"D"|"d" => Direction::Down,
			"L"|"l" => Direction::Left,
			"R"|"r" => Direction::Right,
			_ => panic!("Could not parse direction. Expected 'U', 'D', 'S', or 'R'. Got '{}'", s),
		}
	}
}

pub struct Movement {
	dir: Direction,
	amount: i32,
}

impl fmt::Display for Movement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} {}", self.dir, self.amount)
	}
}

impl Movement {
	pub fn new(dir: Direction, amount: i32) -> Self {
		Movement {dir, amount}
	}
}

struct Knot {
	x: i32,
	y: i32,
	location_history: HashSet::<(i32, i32)>,
}

impl fmt::Display for Knot {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({},{})", self.x, self.y)
	}
}

impl Knot {
	fn new(x: i32, y: i32) -> Self {
		let mut locations = HashSet::new();
		locations.insert((x, y));

		Knot {
			x,
			y,
			location_history: locations,
		}
	}

	fn move_along_x(&mut self, change: i32) {
		self.x += change;
	}

	fn move_along_y(&mut self, change: i32) {
		self.y += change;
	}

	fn move_direction(&mut self, dir: &Direction) {
		match dir {
			Direction::Up 		=> self.y += 1,
			Direction::Down		=> self.y -= 1,
			Direction::Left		=> self.x -= 1,
			Direction::Right	=> self.x += 1,
		}
	}

	fn store_location(&mut self) {
		self.location_history.insert((self.x, self.y));
	}
}

pub struct Rope {
	head: Knot,			// first knot in the rope
	tail: Vec<Knot>,	// the rest of the knots
}

impl fmt::Display for Rope {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		s += format!("({},{})", self.head.x, self.head.y).as_str();
		for tail in &self.tail {
			s += format!("-({},{})", tail.x, tail.y).as_str();
		}
		write!(f, "{}", s)
	}
}

impl Rope {
	pub fn new(num_knots: i32) -> Self {
		if num_knots <= 1 {
			panic!("Rope cannot be less than 2 knots long!");
		}

		// creating tail knots
		let mut tail = vec![];
		for _ in 0..num_knots-1 {
			tail.push(Knot::new(0, 0));
		}

		Rope {
			head: Knot::new(0, 0),
			tail
		}
	}

	pub fn move_head(&mut self, movement: &Movement) {
		for _ in 0..movement.amount {
			// move head
			self.head.move_direction(&movement.dir);
			self.head.store_location();
		
			// move all tails
			for i in 0..self.tail.len() {
				let prev_knot_x = match i {
					0 => self.head.x,
					_ => self.tail[i-1].x,
				};
				let prev_knot_y = match i {
					0 => self.head.y,
					_ => self.tail[i-1].y,
				};

				let x_diff = prev_knot_x - self.tail[i].x;
				let y_diff = prev_knot_y - self.tail[i].y;

				// if the difference in x or y is greater than 1, we need to move the tail
				if x_diff.abs() > 1 || y_diff.abs() > 1 {
					if x_diff == 0 {
						self.tail[i].move_along_y(y_diff / 2);
					}
					else if y_diff == 0 {
						self.tail[i].move_along_x(x_diff / 2);
					}
					else {
						self.tail[i].move_along_x(if x_diff > 0 {1} else {-1}); 
						self.tail[i].move_along_y(if y_diff > 0 {1} else {-1});
					}
				}
				self.tail[i].store_location();
			}
		}
	}

	pub fn get_unique_tail_locations(&self) -> usize {
		self.tail.last().unwrap().location_history.len()
	}
}
