use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<i32>
}

impl Monkey {
    fn new(items: VecDeque<i32>) -> Self {
        Monkey {
            items,
        }
    }

    fn add(&mut self, item: i32) {
        self.items.push_back(item);
    }

    fn remove(&mut self) -> Option<i32> {
        self.items.pop_front()
    }
}

fn main() {
    // initialize all monkeys with their starting items

    // Do 10 rounds
    for _ in 0..20 {
        // handle Monkey 0

        // handle Monkey 1

        // handle Monkey 2

        // handle Monkey 3

        // handle Monkey 4

        // handle Monkey 5

        // handle Monkey 6

        // handle Monkey 7

    }
    
}