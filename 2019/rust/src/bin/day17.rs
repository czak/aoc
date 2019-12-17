#![allow(unused_imports)]
use aoc2019::intcode_redux as intcode;
use std::collections::HashSet;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn to(&self, dir: Dir) -> Pos {
        let Pos { x, y } = *self;
        match dir {
            Dir::Up => Pos { x, y: y - 1 },
            Dir::Down => Pos { x, y: y + 1 },
            Dir::Left => Pos { x: x - 1, y },
            Dir::Right => Pos { x: x + 1, y },
        }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(p: (usize, usize)) -> Self {
        Self {
            x: p.0 as i32,
            y: p.1 as i32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
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

#[derive(Debug, Copy, Clone)]
struct Robot {
    pos: Pos,
    dir: Dir,
}

impl Robot {
    fn navigate(&self, path: &HashSet<Pos>) -> RobotNavigator {
        RobotNavigator {
            path: path.clone(),
            pos: self.pos,
            dir: self.dir,
        }
    }
}

struct RobotNavigator {
    path: HashSet<Pos>,
    pos: Pos,
    dir: Dir,
}

impl Iterator for RobotNavigator {
    type Item = (Pos, Vec<(Dir, Pos)>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = vec![
            (self.dir, self.pos.to(self.dir)),
            (self.dir.turn_left(), self.pos.to(self.dir.turn_left())),
            (self.dir.turn_right(), self.pos.to(self.dir.turn_right())),
        ];
        vec.retain(|(_, pos)| self.path.contains(pos));

        if vec.is_empty() {
            None
        } else {
            let pos = self.pos;
            self.dir = vec[0].0;
            self.pos = vec[0].1;
            Some((pos, vec))
        }
    }
}

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day17.in")),
        ..Default::default()
    };
    ic.mem.resize(5000, 0);

    let s: String = ic.map(|n| char::from(n as u8)).collect();
    let (path, robot) = parse(&s);

    println!("Part 1: {}", intersections(&path, robot));
}

fn intersections(path: &HashSet<Pos>, robot: Robot) -> i32 {
    let set = robot
        .navigate(path)
        .filter(|(_, directions)| directions.len() > 1)
        .map(|(pos, _)| pos)
        .collect::<HashSet<Pos>>();

    set.iter().fold(0, |acc, pos| acc + pos.x * pos.y)
}

fn parse(s: &str) -> (HashSet<Pos>, Robot) {
    let mut path: HashSet<Pos> = HashSet::new();
    let mut robot: Robot = Robot {
        pos: Pos { x: 0, y: 0 },
        dir: Dir::Up,
    };

    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '^' | 'v' | '<' | '>' => {
                    robot = Robot {
                        pos: Pos::from((x, y)),
                        dir: Dir::from(ch),
                    };
                    path.insert(Pos::from((x, y)));
                }
                '#' => {
                    path.insert(Pos::from((x, y)));
                }
                _ => (),
            }
        }
    }

    (path, robot)
}
