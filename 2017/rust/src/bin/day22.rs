use std::io::{self, Read};
use std::collections::HashSet;

type Point = (i32, i32);

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let width: i32 = input.lines().nth(0).unwrap().len() as i32;
    let height: i32 = input.lines().count() as i32;

    let mut infected: HashSet<Point> = HashSet::new();

    for (line, y) in input.lines().zip(-height/2..=height/2) {
        for (node, x) in line.chars().zip(-width/2..=width/2) {
            if node == '#' {
                infected.insert((x, y));
            }
        }
    }

    let mut current: Point = (0, 0);
    let mut direction = Direction::Up;
    let mut infection_count = 0;

    for _ in 0..10000 {
        if infected.contains(&current) {
            direction = match direction {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            };
            infected.remove(&current);
        } else {
            direction = match direction {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
            };
            infected.insert(current);

            infection_count += 1;
        }

        current = match direction {
            Direction::Left => (current.0 - 1, current.1),
            Direction::Up => (current.0, current.1 - 1),
            Direction::Right => (current.0 + 1, current.1),
            Direction::Down => (current.0, current.1 + 1),
        }
    }

    println!("Part 1: {}", infection_count);
}
