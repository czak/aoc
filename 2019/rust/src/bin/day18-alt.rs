#![allow(dead_code, unused_imports)]
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{BTreeMap as Map, BTreeSet as Set};

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
const EX3: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

// 136 steps
const EX4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

type Pos = (usize, usize);

#[derive(Debug)]
struct Tunnel {
    map: Vec<Vec<char>>,
    keys: Set<Pos>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Step(Pos, usize, Set<Step>);

fn main() {
    let (tunnel, pos) = parse(include_str!("../../../in/day18.in"));
    let res = traverse(pos, &tunnel, Set::new(), 0);

    draw(&res, 0);
    println!("Part 1: {}", minimize(&res));
}

fn minimize(steps: &Set<Step>) -> usize {
    if steps.is_empty() {
        return 0;
    }

    steps
        .iter()
        .map(|Step(_, cost, next)| cost + minimize(next))
        .min()
        .unwrap()
}

fn draw(steps: &Set<Step>, level: usize) {
    for Step(pos, cost, next) in steps {
        for _ in 0..level {
            print!("  ");
        }
        println!("{:?} - {}", pos, cost);
        draw(next, level + 1);
    }
}

fn traverse(origin: Pos, tunnel: &Tunnel, collected: Set<Pos>, level: usize) -> Set<Step> {
    // DEBUG
    if level < 7 {
        println!("traverse(origin: {:?}, collected: {:?})", origin, collected);
        println!(
            " - reachable: {:?}",
            reachable_keys(origin, tunnel, &collected)
        );
    }

    let mut res = Set::new();
    for (key_pos, cost) in reachable_keys(origin, tunnel, &collected) {
        let mut collected = collected.clone();
        collected.insert(key_pos);
        let next = traverse(key_pos, tunnel, collected, level + 1);
        res.insert(Step(key_pos, cost, next));
    }
    res
}

// tunnel keys which
// - have not yet been collected
fn reachable_keys(origin: Pos, tunnel: &Tunnel, collected: &Set<Pos>) -> Set<(Pos, usize)> {
    // get a list of open doors
    let open_doors: Set<char> = collected
        .iter()
        .map(|&(x, y)| tunnel.map[y][x].to_ascii_uppercase())
        .collect();

    tunnel
        .keys
        .iter()
        .filter(|key| !collected.contains(&key))
        .copied()
        .filter_map(|key| {
            dijkstra(
                &origin,
                |&p| successors(p, tunnel, collected, &open_doors),
                |&p| p == key,
            )
            .map(|(_, cost)| (key, cost))
        })
        .collect()
}

fn successors(
    (ox, oy): Pos,
    tunnel: &Tunnel,
    collected: &Set<Pos>,
    open_doors: &Set<char>,
) -> Vec<(Pos, usize)> {
    // key is a barrier
    if tunnel.map[oy][ox].is_ascii_lowercase() && !collected.contains(&(ox, oy)) {
        return vec![];
    }

    vec![(ox - 1, oy), (ox + 1, oy), (ox, oy - 1), (ox, oy + 1)]
        .into_iter()
        .filter(|&(x, y)| {
            tunnel.map[y][x] == '.'
                || tunnel.map[y][x].is_ascii_lowercase()
                || open_doors.contains(&tunnel.map[y][x])
        })
        .map(|p| (p, 1))
        .collect()
}

fn parse(s: &str) -> (Tunnel, Pos) {
    let mut tunnel = Tunnel {
        map: vec![],
        keys: Set::new(),
    };

    let mut pos: Pos = (0, 0);

    for (y, line) in s.trim().lines().enumerate() {
        let mut v = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_lowercase() {
                tunnel.keys.insert((x, y));
            }

            if ch == '@' {
                pos = (x, y);
                v.push('.');
            } else {
                v.push(ch);
            }
        }
        tunnel.map.push(v);
    }

    (tunnel, pos)
}
