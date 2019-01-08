use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Node {
    name: &'static str,
    weight: i32,
    children: Vec<&'static str>,
}

fn main() {
    let input = include_str!("../../../in/day07.in");
    let re = Regex::new(r"(\w+) \((\d+)\)(?: -> (.*))?").unwrap();

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let name: &str = caps.get(1).unwrap().as_str();
        let weight: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let children: Vec<&str> = caps.get(3)
            .map(|m| m.as_str().split(", ").collect())
            .unwrap_or(vec![]);

        nodes.insert(name, Node { name, weight, children });
    }

    let root = part1(&nodes);
    println!("Part 1: {}", part1(&nodes));
    println!("Part 2:");

    part2(&nodes, root);
    part2(&nodes, "aazgvmc");
    part2(&nodes, "zuahdoy");
    part2(&nodes, "mfzpvpj");
}

fn part1(nodes: &HashMap<&'static str, Node>) -> &'static str {
    let mut names: HashSet<&str> = nodes.keys().map(|&n| n).collect();

    for node in nodes.values() {
        for c in node.children.iter() {
            names.remove(c);
        }
    }

    names.iter().next().unwrap()
}

fn total_weight(nodes: &HashMap<&'static str, Node>, name: &'static str) -> i32 {
    let node = nodes.get(name).unwrap();
    node.weight + node.children.iter().map(|c| total_weight(nodes, c)).sum::<i32>()
}

fn part2(nodes: &HashMap<&'static str, Node>, root: &'static str) {
    let get_node = |name: &&str| nodes.get(*name).unwrap();

    let node = nodes.get(root).unwrap();
    for child in node.children.iter().map(get_node)  {
        println!("{}: {}/{}", child.name, child.weight, total_weight(nodes, child.name));
    }
    println!("---");
}
