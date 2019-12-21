#![allow(unused_imports)]
use aoc2019::intcode_redux as intcode;
use std::io::{self, Read};

const PROGRAM: &str = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

fn main() {
    // let mut buffer = String::new();
    // io::stdin().read_to_string(&mut buffer).unwrap();

    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day21.in")),
        input: PROGRAM.chars().map(|c| c as i64).collect(),
        ..Default::default()
    };
    ic.mem.resize(5000, 0);

    println!("Part 1: {}", ic.last().unwrap());
}
