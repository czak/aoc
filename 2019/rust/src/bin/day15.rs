// #![allow(unused_imports)]
use aoc2019::intcode_redux as intcode;
use maplit::{hashmap, hashset};
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{HashMap, HashSet};

enum Tile {
    Wall,
    Open,
    Oxygen,
}

#[derive(Copy, Clone)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn to(&self, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::South => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Dir::West => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Dir::East => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day15.in")),
        ..Default::default()
    };

    let mut map: HashMap<Pos, Tile> = hashmap!();
    discover(Pos { x: 0, y: 0 }, &mut ic, &mut map);

    draw(&map);
}

#[allow(dead_code)]
fn draw(map: &HashMap<Pos, Tile>) {
    let xmin = map.keys().min_by_key(|Pos { x, .. }| x).unwrap().x;
    let xmax = map.keys().max_by_key(|Pos { x, .. }| x).unwrap().x;
    let ymin = map.keys().min_by_key(|Pos { y, .. }| y).unwrap().y;
    let ymax = map.keys().max_by_key(|Pos { y, .. }| y).unwrap().y;

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if (x, y) == (0, 0) {
                print!("O");
            } else {
                match map.get(&Pos { x, y }) {
                    Some(Tile::Wall) => print!("#"),
                    Some(Tile::Open) => print!("."),
                    Some(Tile::Oxygen) => print!("$"),
                    _ => print!(" "),
                }
            }
        }
        println!();
    }
}

fn discover(pos: Pos, ic: &mut intcode::Intcode, map: &mut HashMap<Pos, Tile>) {
    // 1 2 3 4
    // N S W E
    for &dir in &[Dir::North, Dir::South, Dir::West, Dir::East] {
        if !map.contains_key(&pos.to(dir)) {
            ic.input.push(dir as i64);
            let res = ic.next().unwrap();
            if res == 2 {
                map.insert(pos.to(dir), Tile::Oxygen);
                discover(pos.to(dir), ic, map);
                // return
                ic.input.push(dir.opposite() as i64);
                ic.next();
            } else if res == 1 {
                map.insert(pos.to(dir), Tile::Open);
                discover(pos.to(dir), ic, map);
                // return
                ic.input.push(dir.opposite() as i64);
                ic.next();
            } else {
                map.insert(pos.to(dir), Tile::Wall);
            }
        }
    }
}
