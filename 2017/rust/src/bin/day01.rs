use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let v: Vec<u32> = s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap()).collect();

    println!("Part 1: {}", part1(&v));
    println!("Part 2: {}", part2(&v));
}

fn part1(v: &[u32]) -> u32 {
    let mut last = v.last().unwrap();
    let mut total = 0;
    for n in v {
        if n == last {
            total += n;
        }
        last = n;
    }
    total
}

fn part2(v: &[u32]) -> u32 {
    let i1 = v.iter();
    let i2 = v.iter().cycle().skip(v.len() / 2);

    i1.zip(i2).filter(|(a, b)| a == b).map(|(a, _)| a).sum()
}
