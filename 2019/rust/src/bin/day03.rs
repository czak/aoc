use std::collections::HashSet;
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

// impl FromStr for Step {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.chars().nth(0) {
//             Some('s') => Ok(Step::Spin(s[1..].parse::<usize>().unwrap())),
//             Some('x') => {
//                 let positions: Vec<usize> = s[1..].split('/').map(|s| s.parse().unwrap()).collect();
//                 Ok(Step::Exchange(positions[0], positions[1]))
//             }
//             Some('p') => {
//                 let n1: char = s.as_bytes()[1] as char;
//                 let n2: char = s.as_bytes()[3] as char;
//                 Ok(Step::Partner(n1, n2))
//             }
//             _ => Err(()),
//         }
//     }
// }

fn navigate(movements: Vec<Movement>) -> HashSet<Point> {
    let mut positions = HashSet::new();
    positions.insert((0, 0));

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // navigate
    for m in movements {
        let (dx, dy) = match m {
            Movement::Up(_) => (0, 1),
            Movement::Down(_) => (0, 1),
            Movement::Left(_) => (0, 1),
            Movement::Right(_) => (0, 1),
        };
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

    let wire1 = read_wire();
    let wire2 = read_wire();

    let points1: HashSet<Point> = navigate(wire1);
    let points2: HashSet<Point> = navigate(wire2);
}
