use std::collections::{HashMap, HashSet};
#[macro_use]
extern crate maplit;

type OrbitMap<'a> = HashMap<&'a str, Vec<&'a str>>;
type ParentMap<'a> = HashMap<&'a str, &'a str>;

fn main() {
    let mut map = OrbitMap::new();
    let mut parent_map = ParentMap::new();
    for (key, value) in include_str!("../../../in/day06.in")
        .lines()
        .map(|l| l.split(')'))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
    {
        map.entry(key).or_insert(vec![]).push(value);
        parent_map.insert(value, key);
    }

    println!("Part 1: {}", count(&map, "COM", 0));
    println!("Part 2: {}", distance(&parent_map, "YOU", "SAN"));
}

fn count(map: &OrbitMap, base: &str, current: u32) -> u32 {
    let mut total = current;
    if let Some(objects) = map.get(base) {
        for o in objects {
            total += count(map, o, current + 1);
        }
    }
    total
}

fn path<'a>(parent_map: &'a ParentMap, mut current: &'a str, end: &str) -> Vec<&'a str> {
    let mut path = Vec::new();
    while current != end {
        current = parent_map.get(current).unwrap();
        path.push(current);
    }
    path
}

fn distance(parent_map: &ParentMap, a: &str, b: &str) -> u32 {
    let a_path: HashSet<&str> = path(&parent_map, a, "COM").iter().cloned().collect();
    let b_path: HashSet<&str> = path(&parent_map, b, "COM").iter().cloned().collect();

    let common_len = a_path.intersection(&b_path).count();

    (a_path.len() + b_path.len() - 2 * common_len) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_map() -> OrbitMap<'static> {
        hashmap! {
            "COM" => vec!["B"],
            "B" => vec!["G", "C"],
            "G" => vec!["H"],
            "C" => vec!["D"],
            "D" => vec!["I", "E"],
            "E" => vec!["F", "J"],
            "J" => vec!["K"],
            "K" => vec!["L"],
        }
    }

    #[test]
    fn test_count_example() {
        assert_eq!(42, count(&example_map(), "COM", 0));
    }
}
