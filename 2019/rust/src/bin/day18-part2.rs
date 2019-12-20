#![allow(dead_code, unused_imports)]
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{BTreeSet, HashMap};

type Pos = (usize, usize);

struct Segment {
    cost: usize,
    keys: BTreeSet<char>,
    doors: BTreeSet<char>,
}

#[derive(Debug)]
struct Step(char, usize, Vec<Step>);

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?}, {:?})", self.cost, self.keys, self.doors)
    }
}

const EX1: &str = "
#######
#a.#Cd#
##0#1##
#######
##2#3##
#cB#Ab#
#######";

fn main() {
    let map = parse(EX1);
    assert_eq!(8, find_path(&map));

    let map = parse(include_str!("../../../in/day18-part2.in"));
    dbg!(find_path(&map));
}

fn find_path(map: &HashMap<char, HashMap<char, Segment>>) -> usize {
    type State = ([char; 4], BTreeSet<char>);

    let initial_state: State = (['0', '1', '2', '3'], BTreeSet::new());
    let num_keys = map.keys().len() - 4;

    fn successors(
        state: &State,
        map: &HashMap<char, HashMap<char, Segment>>,
    ) -> Vec<(State, usize)> {
        let current = state.0;
        let collected = &state.1;

        // get keys reachable to all robots

        let mut v = vec![];

        if let Some(directions) = map.get(&current[0]) {
            let mut v0 = reachable_keys(directions, collected)
                .iter()
                .map(|&(key, cost)| {
                    let mut collected = collected.clone();
                    collected.insert(key);
                    (([key, current[1], current[2], current[3]], collected), cost)
                })
                .collect();
            v.append(&mut v0);
        }

        if let Some(directions) = map.get(&current[1]) {
            let mut v1 = reachable_keys(directions, collected)
                .iter()
                .map(|&(key, cost)| {
                    let mut collected = collected.clone();
                    collected.insert(key);
                    (([current[0], key, current[2], current[3]], collected), cost)
                })
                .collect();
            v.append(&mut v1);
        }

        if let Some(directions) = map.get(&current[2]) {
            let mut v2 = reachable_keys(directions, collected)
                .iter()
                .map(|&(key, cost)| {
                    let mut collected = collected.clone();
                    collected.insert(key);
                    (([current[0], current[1], key, current[3]], collected), cost)
                })
                .collect();
            v.append(&mut v2);
        }

        if let Some(directions) = map.get(&current[3]) {
            let mut v3 = reachable_keys(directions, collected)
                .iter()
                .map(|&(key, cost)| {
                    let mut collected = collected.clone();
                    collected.insert(key);
                    (([current[0], current[1], current[2], key], collected), cost)
                })
                .collect();
            v.append(&mut v3);
        }

        v
    }

    dijkstra(
        &initial_state,
        |state| successors(state, map),
        |state| state.1.len() == num_keys,
    )
    .unwrap()
    .1
}

fn reachable_keys(
    directions: &HashMap<char, Segment>,
    collected: &BTreeSet<char>,
) -> Vec<(char, usize)> {
    let mut v = vec![];
    for (&key, Segment { cost, keys, doors }) in directions {
        if collected.contains(&key) {
            continue;
        }

        if collected.is_superset(&keys) && collected.is_superset(&doors) {
            v.push((key, *cost));
        }
    }
    v
}

// {
//     '@': {
//         'a': (2, {}, {}),
//         'b': (4, {}, {'a'}),
//     },
//     'a': {
//         'b': (6, {}, {'a'}),
//     },
//     'b': {
//         'a': (6, {}, {'a'}),
//     },
// }
fn parse(s: &str) -> HashMap<char, HashMap<char, Segment>> {
    let mut start: [Pos; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];
    let mut map: Vec<Vec<char>> = vec![];
    let mut keys: HashMap<Pos, char> = HashMap::new();

    for (y, line) in s.trim().lines().enumerate() {
        let mut v = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_lowercase() {
                keys.insert((x, y), ch);
            }

            if ch == '0' {
                start[0] = (x, y);
            }
            if ch == '1' {
                start[1] = (x, y);
            }
            if ch == '2' {
                start[2] = (x, y);
            }
            if ch == '3' {
                start[3] = (x, y);
            }

            v.push(ch);
        }
        map.push(v);
    }

    let mut res: HashMap<char, HashMap<char, Segment>> = HashMap::new();

    for ((ax, ay), a) in vec![
        (start[0], '0'),
        (start[1], '1'),
        (start[2], '2'),
        (start[3], '3'),
    ]
    .into_iter()
    .chain(keys.clone().into_iter())
    {
        let mut directions: HashMap<char, Segment> = HashMap::new();
        for (&(bx, by), &b) in keys.iter() {
            if a == b {
                continue;
            };
            if let Some(segment) = cost((ax, ay), (bx, by), &map) {
                directions.insert(b, segment);
            }
        }
        res.insert(a, directions);
    }

    res
}

fn cost(a: Pos, b: Pos, map: &Vec<Vec<char>>) -> Option<Segment> {
    let mut keys = BTreeSet::new();
    let mut doors = BTreeSet::new();

    if let Some((path, cost)) = dijkstra(&a, |&p| successors(p, map), |&p| p == b) {
        // collect keys & doors between
        for &(x, y) in &path[1..path.len() - 1] {
            if map[y][x].is_ascii_lowercase() {
                keys.insert(map[y][x]);
            }
            if map[y][x].is_ascii_uppercase() {
                doors.insert(map[y][x].to_ascii_lowercase());
            }
        }

        Some(Segment { cost, keys, doors })
    } else {
        None
    }
}

fn successors((x, y): Pos, map: &Vec<Vec<char>>) -> Vec<(Pos, usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|&(x, y)| map[y][x] != '#')
        .map(|p| (p, 1))
        .collect()
}
