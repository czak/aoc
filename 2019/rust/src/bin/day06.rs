use std::collections::HashMap;
#[macro_use]
extern crate maplit;

type OrbitMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let mut map = OrbitMap::new();
    for (key, value) in include_str!("../../../in/day06.in")
        .lines()
        .map(|l| l.split(')'))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
    {
        map.entry(key).or_insert(vec![]).push(value);
    }

    println!("Part 1: {}", count(&map, "COM", 0));
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
