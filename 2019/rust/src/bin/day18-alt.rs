#![allow(dead_code, unused_imports)]
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{BTreeMap as Map, BTreeSet as Set};

// 8 steps (keys: a, b)
const EX1: &str = "
#########
#b.A.@.a#
#########";

type Pos = (usize, usize);

#[derive(Debug)]
struct Tunnel {
    map: Vec<Vec<char>>,
    keys: Set<Pos>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Step(Pos, usize, Set<Step>);

fn main() {
    let (tunnel, pos) = parse(EX1);
    let res = traverse(pos, &tunnel, Set::new());

    println!("{:?}", res);
}

fn traverse(origin: Pos, tunnel: &Tunnel, collected: Set<Pos>) -> Set<Step> {
    println!("traverse(origin: {:?}, collected: {:?})", origin, collected);
    println!(
        " - reachable: {:?}",
        reachable_keys(origin, tunnel, &collected)
    );

    let mut res = Set::new();
    for (key_pos, cost) in reachable_keys(origin, tunnel, &collected) {
        let mut collected = collected.clone();
        collected.insert(key_pos);
        let next = traverse(key_pos, tunnel, collected);
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
