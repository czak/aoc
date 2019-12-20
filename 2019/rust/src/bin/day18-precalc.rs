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

// MOJE
const EX0: &str = "
###########
#c.A.b.@.a#
###########";

// 8 steps (keys: a, b)
const EX1: &str = "
#########
#b.A.@.a#
#########";

// 86 steps (keys: a, b, c, d, e, f)
const EX2: &str = "
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

// 132 steps
const EX3: &str = "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

// 136 steps
const EX4: &str = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

fn main() {
    let map = parse(EX1);
    assert_eq!(8, find_path(&map));

    let map = parse(EX2);
    assert_eq!(86, find_path(&map));

    let map = parse(EX3);
    assert_eq!(132, find_path(&map));

    let map = parse(EX4);
    assert_eq!(136, find_path(&map));

    let map = parse(include_str!("../../../in/day18.in"));
    dbg!(find_path(&map));
}

fn find_path(map: &HashMap<char, HashMap<char, Segment>>) -> usize {
    type State = (char, BTreeSet<char>);

    let initial_state: State = ('@', BTreeSet::new());
    let num_keys = map.keys().len() - 1;

    fn successors(
        state: &State,
        map: &HashMap<char, HashMap<char, Segment>>,
    ) -> Vec<(State, usize)> {
        let current = state.0;
        let collected = &state.1;
        reachable_keys(map.get(&current).unwrap(), collected)
            .iter()
            .map(|&(key, cost)| {
                let mut collected = collected.clone();
                collected.insert(key);
                ((key, collected), cost)
            })
            .collect()
    }

    dijkstra(
        &initial_state,
        |state| successors(state, map),
        |state| state.1.len() == num_keys,
    )
    .unwrap()
    .1
}

// fn minimize(
//     map: &HashMap<char, HashMap<char, Segment>>,
//     current: char,
//     collected: BTreeSet<char>,
// ) -> usize {
//     reachable_keys(map.get(&current).unwrap(), &collected)
//         .par_iter()
//         .map(|&(key, cost)| {
//             let mut collected = collected.clone();
//             collected.insert(key);
//             cost + minimize(map, key, collected)
//         })
//         .min()
//         .unwrap_or(0)
// }
//
// fn upper(
//     map: &HashMap<char, HashMap<char, Segment>>,
//     current: char,
//     collected: BTreeSet<char>,
//     total: usize,
// ) -> usize {
//     println!("Minimize: {}, {}", current, total);
//
//     if let Some(&(best, min)) = reachable_keys(map.get(&current).unwrap(), &collected)
//         .iter()
//         .min_by_key(|&(_, cost)| cost)
//     {
//         let mut collected = collected.clone();
//         collected.insert(best);
//         return upper(map, best, collected, total + min);
//     } else {
//         return total;
//     }
// }

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

fn reachable_keys2(
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
    let mut start: Pos = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];
    let mut keys: HashMap<Pos, char> = HashMap::new();

    for (y, line) in s.trim().lines().enumerate() {
        let mut v = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_lowercase() {
                keys.insert((x, y), ch);
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
    let mut keys = BTreeSet::new();
    let mut doors = BTreeSet::new();

    let (path, cost) = dijkstra(&a, |&p| successors(p, map), |&p| p == b).unwrap();

    // collect keys & doors between
    for &(x, y) in &path[1..path.len() - 1] {
        if map[y][x].is_ascii_lowercase() {
            keys.insert(map[y][x]);
        }
        if map[y][x].is_ascii_uppercase() {
            doors.insert(map[y][x].to_ascii_lowercase());
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
