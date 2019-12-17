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

    fn after(self, step: Step) -> Self {
        match step {
            Step::Forward => self,
            Step::TurnLeft => self.turn_left(),
            Step::TurnRight => self.turn_right(),
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Step {
    Forward,
    TurnLeft,
    TurnRight,
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

impl RobotNavigator {
    fn step(&mut self, step: Step) {
        match step {
            Step::Forward => {
                self.pos = self.pos.to(self.dir);
            }
            Step::TurnLeft | Step::TurnRight => {
                self.dir = self.dir.after(step);
            }
        }
    }

    #[allow(dead_code)]
    fn draw(&mut self) {
        let way = self.map(|(pos, steps)| (pos, steps[0])).collect::<Vec<_>>();
        let mut dist = 0;
        for (pos, step) in &way {
            if *step == Step::Forward {
                dist += 1;
            } else {
                if dist > 0 {
                    println!("({:2},{:2}), {}", pos.x, pos.y, dist);
                    dist = 0;
                }
                println!(
                    "({:2},{:2}), {}",
                    pos.x,
                    pos.y,
                    if *step == Step::TurnLeft { 'L' } else { 'R' }
                );
            }
        }
        if dist > 0 {
            println!(
                "({:2},{:2}), {:?}",
                way[way.len() - 1].0.x,
                way[way.len() - 1].0.y,
                dist
            );
        }
    }
}

impl Iterator for RobotNavigator {
    type Item = (Pos, Vec<Step>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = vec![Step::Forward, Step::TurnLeft, Step::TurnRight];
        vec.retain(|&step| self.path.contains(&self.pos.to(self.dir.after(step))));

        if vec.is_empty() {
            None
        } else {
            let pos = self.pos;
            self.step(vec[0]); // pick first possible direction, forward preferred
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
    assert_eq!(8408, intersections(&path, robot));

    // part 2

    // robot.navigate(&path).draw();

    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day17.in")),
        input: include_str!("../../../in/day17-drawing.txt")
            .chars()
            .map(|c| c as i64)
            .collect(),
        ..Default::default()
    };
    ic.mem[0] = 2;
    ic.mem.resize(5000, 0);

    let res = ic.last().unwrap();
    println!("Part 2: {}", res);
    assert_eq!(1168948, res);
}

fn intersections(path: &HashSet<Pos>, robot: Robot) -> i32 {
    let set = robot
        .navigate(path)
        .filter(|(_, steps)| steps.len() == 3)
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
