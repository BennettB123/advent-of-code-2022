use std::collections::{VecDeque};

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    items_inspected: u32,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            items_inspected: 0
        }
    }

    fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    fn give(&mut self, item: i64) {
        self.items.push_back(item);
    }

    fn remove(&mut self) -> i64 {
        self.items_inspected += 1;
        self.items.pop_front().unwrap()
    }
}

fn main() {
    // initialize all monkeys with their starting items
    let mut monkey_0 = Monkey::new();
    monkey_0.give(63);
    monkey_0.give(57);

    let mut monkey_1 = Monkey::new();
    monkey_1.give(82);
    monkey_1.give(66);
    monkey_1.give(87);
    monkey_1.give(78);
    monkey_1.give(77);
    monkey_1.give(92);
    monkey_1.give(83);

    let mut monkey_2 = Monkey::new();
    monkey_2.give(97);
    monkey_2.give(53);
    monkey_2.give(53);
    monkey_2.give(85);
    monkey_2.give(58);
    monkey_2.give(54);

    let mut monkey_3 = Monkey::new();
    monkey_3.give(50);

    let mut monkey_4 = Monkey::new();
    monkey_4.give(64);
    monkey_4.give(69);
    monkey_4.give(52);
    monkey_4.give(65);
    monkey_4.give(73);

    let mut monkey_5 = Monkey::new();
    monkey_5.give(57);
    monkey_5.give(91);
    monkey_5.give(65);

    let mut monkey_6 = Monkey::new();
    monkey_6.give(67);
    monkey_6.give(91);
    monkey_6.give(84);
    monkey_6.give(78);
    monkey_6.give(60);
    monkey_6.give(69);
    monkey_6.give(99);
    monkey_6.give(83);

    let mut monkey_7 = Monkey::new();
    monkey_7.give(58);
    monkey_7.give(78);
    monkey_7.give(69);
    monkey_7.give(65);

    // Do 20 rounds
    for _ in 0..20 {
        // handle Monkey 0
        while monkey_0.has_items() {
            let mut item = monkey_0.remove();
            item *= 11;
            item /= 3;
            if item % 7 == 0 {
                monkey_6.give(item);
            }
            else {
                monkey_2.give(item);
            }
        }

        // handle Monkey 1
        while monkey_1.has_items() {
            let mut item = monkey_1.remove();
            item += 1;
            item /= 3;
            if item % 11 == 0 {
                monkey_5.give(item);
            }
            else {
                monkey_0.give(item);
            }
        }

        // handle Monkey 2
        while monkey_2.has_items() {
            let mut item = monkey_2.remove();
            item *= 7;
            item /= 3;
            if item % 13 == 0 {
                monkey_4.give(item);
            }
            else {
                monkey_3.give(item);
            }
        }

        // handle Monkey 3
        while monkey_3.has_items() {
            let mut item = monkey_3.remove();
            item += 3;
            item /= 3;
            if item % 3 == 0 {
                monkey_1.give(item);
            }
            else {
                monkey_7.give(item);
            }
        }

        // handle Monkey 4
        while monkey_4.has_items() {
            let mut item = monkey_4.remove();
            item += 6;
            item /= 3;
            if item % 17 == 0 {
                monkey_3.give(item);
            }
            else {
                monkey_7.give(item);
            }
        }

        // handle Monkey 5
        while monkey_5.has_items() {
            let mut item = monkey_5.remove();
            item += 5;
            item /= 3;
            if item % 2 == 0 {
                monkey_0.give(item);
            }
            else {
                monkey_6.give(item);
            }
        }

        // handle Monkey 6
        while monkey_6.has_items() {
            let mut item = monkey_6.remove();
            item *= item;
            item /= 3;
            if item % 5 == 0 {
                monkey_2.give(item);
            }
            else {
                monkey_4.give(item);
            }
        }

        // handle Monkey 7
        while monkey_7.has_items() {
            let mut item = monkey_7.remove();
            item += 7;
            item /= 3;
            if item % 19 == 0 {
                monkey_5.give(item);
            }
            else {
                monkey_1.give(item);
            }
        }
    }

    // Part 1 calculations
    let mut inspection_counts: Vec<u32> = vec![monkey_0.items_inspected,
                                               monkey_1.items_inspected,
                                               monkey_2.items_inspected,
                                               monkey_3.items_inspected,
                                               monkey_4.items_inspected,
                                               monkey_5.items_inspected,
                                               monkey_6.items_inspected,
                                               monkey_7.items_inspected];
    inspection_counts.sort();
    let part_one_monkey_business: u32 = inspection_counts.iter().rev().take(2).product();

    // Part 2 calculations
    

    // Print answers
    println!("################################");
	println!("#### Advent of Code, Day 11 ####");
	println!("################################");
    println!("The two monkeys with the most inspected items had a monkey business of {}", part_one_monkey_business);
}