use std::io;
use std::io::prelude::*;

fn main() {
    let parse_line = |l: String| {
        l.split_whitespace().filter_map(|s| s.parse().ok()).collect()
    };

    let lines: Vec<Vec<i32>> = io::stdin().lock()
        .lines()
        .filter_map(|l| l.map(parse_line).ok())
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for line in lines {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for &n in line {
            if n < min { min = n }
            if n > max { max = n }
        }
        total += max - min;
    }
    total
}

fn part2(lines: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for line in lines {
        for &a in line {
            for &b in line {
                if a != b && a % b == 0 {
                    total += a / b;
                }
            }
        }
    }

    total
}
