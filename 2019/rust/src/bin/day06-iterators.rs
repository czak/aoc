#[cfg(test)]
#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet};

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

    assert_eq!(158090, count(&map, "COM", 0));
    assert_eq!(241, distance(&parent_map, "YOU", "SAN"));

    println!("Part 1: {}", count(&map, "COM", 0));
    println!("Part 2: {}", distance(&parent_map, "YOU", "SAN"));
}

fn count(map: &OrbitMap, base: &str, current: u32) -> u32 {
    current
        + map.get(base).map_or(0, |objects| {
            objects.iter().map(|o| count(map, o, current + 1)).sum()
        })
}

fn path<'a>(parent_map: &'a ParentMap, object: &'a str) -> HashSet<&'a str> {
    // TODO: Simpler way to get from Option<&&str> to Option<&str>?
    std::iter::successors(Some(object), |o| parent_map.get(o).map(|&o| o)).collect()
}

fn distance(parent_map: &ParentMap, a: &str, b: &str) -> u32 {
    let a_path = path(&parent_map, a);
    let b_path = path(&parent_map, b);

    let common_len = a_path.intersection(&b_path).count();

    (a_path.len() + b_path.len() - 2 * common_len - 2) as u32
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
