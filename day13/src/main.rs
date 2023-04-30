// Advent of Code 2022
// Day 13

// Notes: This was a fun one. I need some inspiration for parsing the input into packets. I used a guide on Github from jcollard (https://github.com/jcollard/AdventOfCode2022/tree/main/Day13-Guide)
//	This puzzle was was a great way to demonstrate how powerful rust's enums are.

use std::{collections::VecDeque, env, fs};

#[derive(PartialEq)]
enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(input: &String) -> Self {
        let mut data: VecDeque<char> = input.chars().collect();
        Self::parse_list(&mut data)
    }

    fn parse_list(data: &mut VecDeque<char>) -> Packet {
        let mut list: Vec<Packet> = vec![];

        // remove leading '['
        data.pop_front();

        while data.front().unwrap() != &']' {
            if data.front().unwrap() == &',' {
                data.pop_front();
            }
            list.push(Self::parse_element(data));
        }

        // remove ']' from list
        data.pop_front();
        Packet::List(list)
    }

    fn parse_element(data: &mut VecDeque<char>) -> Packet {
        let next = data.front().unwrap();
        if next.is_ascii_digit() {
            return Self::parse_int(data);
        } else if next == &'[' {
            return Self::parse_list(data);
        } else {
            panic!(
                "Error parsing packet element. Expected digit or '[', but found {:?}",
                data
            );
        }
    }

    fn parse_int(data: &mut VecDeque<char>) -> Packet {
        let mut token = String::new();
        while data.front().unwrap().is_ascii_digit() {
            token.push(data.pop_front().unwrap());
        }

        Packet::Value(token.parse().unwrap())
    }

    // attempts to recreate the input packet (commas are a bit messed up, but close enough...)
    fn to_string(&self) -> String {
        let mut s = String::new();
        match self {
            Packet::Value(val) => {
                s += &val.to_string();
                s += ","
            }
            Packet::List(list) => {
                s += "[";
                for element in list {
                    s += &element.to_string();
                }
                s += "],";
            }
        }

        s
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct PairOfPackets {
    p1: Packet,
    p2: Packet,
}

impl PairOfPackets {
    #[allow(dead_code)]
    fn print(&self) {
        println!("p1: {}", self.p1);
        println!("p2: {}", self.p2);
    }
}

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

    let packet_pairs = parse_file(&file_contents);

    // part 1
    let mut sum = 0;
    let mut index = 1;
    for pair in &packet_pairs {
        if is_in_order(&pair.p1, &pair.p2) {
            sum += index;
        }
        index += 1;
    }

    // part 2
    let mut all_packets: Vec<Packet> = vec![];
    for pair in packet_pairs {
        all_packets.push(pair.p1);
        all_packets.push(pair.p2);
    }
    all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(2)])]));
    all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(6)])]));
    sort_packets(&mut all_packets);
    let divider_1_index = all_packets
        .iter()
        .position(|p| p == &Packet::List(vec![Packet::List(vec![Packet::Value(2)])]))
        .unwrap()
        + 1;
    let divider_2_index = all_packets
        .iter()
        .position(|p| p == &Packet::List(vec![Packet::List(vec![Packet::Value(6)])]))
        .unwrap()
        + 1;

    // print answers
    println!("################################");
    println!("#### Advent of Code, Day 13 ####");
    println!("################################");
    println!(
        "Part 1 - Sum of the indices of correctly ordered packets: {}",
        sum
    );
    println!(
        "Part 2 - Product of indicies of divider packets in sorted list of packets: {}",
        divider_1_index * divider_2_index
    );
}

fn parse_file(contents: &String) -> Vec<PairOfPackets> {
    let mut pairs = vec![];

    // split each pair by splitting on double newline (input file is CRLF)
    for pair_str in contents.split("\r\n\r\n") {
        let pair_split: Vec<&str> = pair_str.lines().collect();
        pairs.push(PairOfPackets {
            p1: Packet::parse(&pair_split.first().unwrap().to_string()),
            p2: Packet::parse(&pair_split.last().unwrap().to_string()),
        })
    }

    pairs
}

fn is_in_order(left: &Packet, right: &Packet) -> bool {
    compare_elements(left, right) <= 0
}

fn compare_elements(left: &Packet, right: &Packet) -> i32 {
    match (left, right) {
        (Packet::Value(l), Packet::Value(r)) => {
            if l == r {
                0
            } else if l - r > 0 {
                1
            } else {
                -1
            }
        }
        (Packet::List(l), Packet::List(r)) => compare_lists(&l, &r),
        (Packet::Value(l), Packet::List(r)) => compare_lists(&vec![Packet::Value(*l)], &r),
        (Packet::List(l), Packet::Value(r)) => compare_lists(&l, &vec![Packet::Value(*r)]),
    }
}

fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> i32 {
    for i in 0..std::cmp::min(left.len(), right.len()) {
        let diff = compare_elements(&left[i], &right[i]);
        if diff < 0 {
            return -1;
        } else if diff > 0 {
            return 1;
        }
    }

    if left.len() == right.len() {
        0
    } else if left.len() as i32 - right.len() as i32 > 0 {
        1
    } else {
        -1
    }
}

// simple bubble sort
fn sort_packets(packets: &mut Vec<Packet>) {
    for _ in 0..packets.len() {
        for i in 0..packets.len() - 1 {
            if !is_in_order(&packets[i], &packets[i + 1]) {
                packets.swap(i, i + 1);
            }
        }
    }
}
