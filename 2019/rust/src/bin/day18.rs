#![allow(dead_code)]
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::{HashMap, HashSet};

// 8 steps (keys: a, b)
const EX1: &str = "#########
#b.A.@.a#
#########";

// 86 steps (keys: a, b, c, d, e, f)
const EX2: &str = "########################
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

const EX0: &str = "
#############
#c.B.b...@.a#
#############";

// {
//   a => Some {
//     b => Some {
//       c => None
//     }
//   }
//   b => Some {
//     a => Some {
//       c => None,
//     },
//     c => Some {
//       a => None,
//     }
//   }
// }

#[derive(Clone)]
struct Tunnel {
    map: Vec<Vec<char>>,
    keys: HashMap<Pos, char>,
    doors: HashMap<char, Pos>,
}

type Pos = (usize, usize);

fn main() {
    let (mut tunnel, mut pos) = parse(EX4);

    let mut total = 0;
    while tunnel.keys.len() > 0 {
        let reachable = reachable_keys(pos, &tunnel);
        // TODO: if multiple, split
        let idx = 0;
        let key = reachable[idx];
        total += collect(pos, key, &mut tunnel);
        pos = key;
    }

    dbg!(total);
}

fn collect(pos: Pos, key: Pos, tunnel: &mut Tunnel) -> i32 {
    let (_path, steps) = dijkstra(&pos, |&p| successors(p, tunnel), |&p| p == key).unwrap();
    // unlock door
    let door_name = tunnel.keys.get(&key).unwrap().to_ascii_uppercase();
    if let Some(&(dx, dy)) = tunnel.doors.get(&door_name) {
        tunnel.map[dy][dx] = '.';
    }
    // remove key
    tunnel.keys.remove(&key);
    tunnel.map[key.1][key.0] = '.';
    steps
}

fn reachable_keys(pos: Pos, tunnel: &Tunnel) -> Vec<Pos> {
    tunnel
        .keys
        .keys()
        .copied()
        .filter(|key| dijkstra(&pos, |&p| successors(p, tunnel), |p| p == key).is_some())
        .collect()
}

fn successors((ox, oy): Pos, tunnel: &Tunnel) -> Vec<(Pos, i32)> {
    // key is a barrier
    if tunnel.map[oy][ox].is_ascii_lowercase() {
        return vec![];
    }

    vec![(ox - 1, oy), (ox + 1, oy), (ox, oy - 1), (ox, oy + 1)]
        .into_iter()
        .filter(|&(x, y)| tunnel.map[y][x] == '.' || tunnel.map[y][x].is_ascii_lowercase())
        .map(|p| (p, 1))
        .collect()
}

fn parse(s: &str) -> (Tunnel, Pos) {
    let mut tunnel = Tunnel {
        map: vec![],
        keys: HashMap::new(),
        doors: HashMap::new(),
    };

    let mut pos: Pos = (0, 0);

    for (y, line) in s.trim().lines().enumerate() {
        let mut v = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_lowercase() {
                tunnel.keys.insert((x, y), ch);
            } else if ch.is_ascii_uppercase() {
                tunnel.doors.insert(ch, (x, y));
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
