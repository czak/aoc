use aoc2019::intcode_redux as intcode;
use std::collections::HashSet;

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day13.in")),
        ..Default::default()
    };

    ic.mem.resize(10000, 0);

    println!("Part 1: {}", part1(ic.clone()));
}

fn part1(ic: intcode::Intcode) -> usize {
    let output: Vec<i64> = ic.collect();
    let mut blocks: HashSet<(i64, i64)> = HashSet::new();

    for chunk in output.chunks(3) {
        let x = chunk[0];
        let y = chunk[1];
        let id = chunk[2];

        if id == 2 {
            blocks.insert((x, y));
        } else {
            blocks.remove(&(x, y));
        }
    }

    blocks.len()
}
