#![allow(unused_imports)]
use aoc2019::intcode_redux as intcode;
use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Unknown dir"),
        }
    }
}

#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    dir: Dir,
}

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day17.in")),
        ..Default::default()
    };
    ic.mem.resize(5000, 0);

    let s: String = ic.map(|n| char::from(n as u8)).collect();
    let (scaffold, robot) = parse(&s);

    println!("{}", s);
    dbg!(robot);
}

fn parse(s: &str) -> (Vec<Vec<char>>, Robot) {
    let mut scaffold: Vec<Vec<char>> = vec![];
    let mut robot: Robot = Robot {
        x: 0,
        y: 0,
        dir: Dir::Up,
    };

    for (y, line) in s.lines().enumerate() {
        let mut v = vec![];
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '^' | 'v' | '<' | '>' => {
                    robot = Robot {
                        x,
                        y,
                        dir: Dir::from(ch),
                    };
                    v.push('#');
                }
                _ => v.push(ch),
            }
            v.push(ch);
        }
        scaffold.push(v);
    }

    (scaffold, robot)
}
