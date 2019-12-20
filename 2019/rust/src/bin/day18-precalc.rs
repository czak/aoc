#![allow(dead_code, unused_imports)]
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

struct Segment {
    cost: usize,
    keys: HashSet<char>,
    doors: HashSet<char>,
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?}, {:?})", self.cost, self.keys, self.doors)
    }
}

// 8 steps (keys: a, b)
const EX1: &str = "
#########
#b.A.@.a#
#########";

const EX4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

fn main() {
    let map = parse(include_str!("../../../in/day18.in"));
    dbg!(map);
}

// @ => {
//   a => (2, []),
//   b => (4, [A]),
// },
// a => {
//   b => (6, [A]),
// },
// b => {
//   a => (6, [A]),
// }
fn parse(s: &str) -> HashMap<char, HashMap<char, Segment>> {
    let mut start: Pos = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];
    let mut keys: HashMap<Pos, char> = HashMap::new();
    let mut doors: HashMap<Pos, char> = HashMap::new();

    for (y, line) in s.trim().lines().enumerate() {
        let mut v = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_lowercase() {
                keys.insert((x, y), ch);
            }
            if ch.is_ascii_uppercase() {
                doors.insert((x, y), ch);
            }
            if ch == '@' {
                start = (x, y);
            }

            v.push(ch);
        }
        map.push(v);
    }

    let mut res: HashMap<char, HashMap<char, Segment>> = HashMap::new();

    for ((ax, ay), a) in std::iter::once((start, '@')).chain(keys.clone().into_iter()) {
        let mut directions: HashMap<char, Segment> = HashMap::new();
        for (&(bx, by), &b) in keys.iter() {
            if a == b {
                continue;
            };
            directions.insert(b, cost((ax, ay), (bx, by), &map));
        }
        res.insert(a, directions);
    }

    res
}

fn cost(a: Pos, b: Pos, map: &Vec<Vec<char>>) -> Segment {
    let mut keys = HashSet::new();
    let mut doors = HashSet::new();

    let (path, cost) = dijkstra(&a, |&p| successors(p, map), |&p| p == b).unwrap();

    // collect keys & doors between
    for &(x, y) in &path[1..path.len() - 1] {
        if map[y][x].is_ascii_lowercase() {
            keys.insert(map[y][x]);
        }
        if map[y][x].is_ascii_uppercase() {
            doors.insert(map[y][x]);
        }
    }

    Segment { cost, keys, doors }
}

fn successors((x, y): Pos, map: &Vec<Vec<char>>) -> Vec<(Pos, usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|&(x, y)| map[y][x] != '#')
        .map(|p| (p, 1))
        .collect()
}
