use std::io::{self, Read};
use std::collections::HashMap;

type Point = (i32, i32);

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

enum State {
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let width: i32 = input.lines().nth(0).unwrap().len() as i32;
    let height: i32 = input.lines().count() as i32;

    let mut infected: HashMap<Point, State> = HashMap::new();

    for (line, y) in input.lines().zip(-height/2..=height/2) {
        for (node, x) in line.chars().zip(-width/2..=width/2) {
            if node == '#' {
                infected.insert((x, y), State::Infected);
            }
        }
    }

    let mut current: Point = (0, 0);
    let mut direction = Direction::Up;
    let mut infection_count = 0;

    for _ in 0..10000000 {
        let state = infected.get(&current);

        // 1. new direction
        direction = match state {
            None => direction.turn_left(),
            Some(&State::Weakened) => direction,
            Some(&State::Infected) => direction.turn_right(),
            Some(&State::Flagged) => direction.reverse(),
        };

        // 2. new state
        match state {
            None => {
                infected.insert(current, State::Weakened);
            },
            Some(&State::Weakened) => {
                infected.insert(current, State::Infected);
                infection_count += 1;
            },
            Some(&State::Infected) => {
                infected.insert(current, State::Flagged);
            },
            Some(&State::Flagged) => {
                infected.remove(&current);
            },
        };

        // 3. move
        current = match direction {
            Direction::Left => (current.0 - 1, current.1),
            Direction::Up => (current.0, current.1 - 1),
            Direction::Right => (current.0 + 1, current.1),
            Direction::Down => (current.0, current.1 + 1),
        };
    }

    println!("Part 2: {}", infection_count);
}
