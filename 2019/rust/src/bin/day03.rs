use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

type Point = (i32, i32);

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distance = s[1..].parse::<u32>().unwrap();
        match s.chars().nth(0) {
            Some('U') => Ok(Movement::Up(distance)),
            Some('D') => Ok(Movement::Down(distance)),
            Some('L') => Ok(Movement::Left(distance)),
            Some('R') => Ok(Movement::Right(distance)),
            _ => Err(()),
        }
    }
}

fn navigate(movements: Vec<Movement>) -> HashMap<Point, u32> {
    let mut positions = HashMap::new();
    positions.insert((0, 0), 0);

    let mut step = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // navigate
    for m in movements {
        let (dx, dy, d) = match m {
            Movement::Up(d) => (0, 1, d),
            Movement::Down(d) => (0, -1, d),
            Movement::Left(d) => (-1, 0, d),
            Movement::Right(d) => (1, 0, d),
        };

        for _ in 0..d {
            step += 1;
            x += dx;
            y += dy;
            positions.entry((x, y)).or_insert(step);
        }
    }

    positions
}

fn read_wire() -> Vec<Movement> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

fn main() {
    assert_eq!(Ok(Movement::Right(992)), "R992".parse::<Movement>());

    let wire1: HashMap<Point, u32> = navigate(read_wire());
    let wire2: HashMap<Point, u32> = navigate(read_wire());

    let points1: HashSet<Point> = wire1.keys().cloned().collect();
    let points2: HashSet<Point> = wire2.keys().cloned().collect();

    let intersections: HashSet<Point> = points1
        .intersection(&points2)
        .cloned()
        .filter(|&i| i != (0, 0))
        .collect();

    let mut min = 1_000_000;
    for i in &intersections {
        let dist = i.0.abs() + i.1.abs();
        if dist < min {
            min = dist;
        }
    }

    println!("Part 1: {}", min);
    assert_eq!(403, min);

    let mut min = 1_000_000;
    for i in &intersections {
        let dist = wire1.get(i).unwrap() + wire2.get(i).unwrap();
        if dist < min {
            min = dist;
        }
    }

    println!("Part 2: {}", min);
    assert_eq!(4158, min);
}
