extern crate regex;

use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    name: String,
    weight: i32,
    subtree: i32,
    children: Vec<Node>,
}

fn parse(s: &str, re1: &Regex, re2: &Regex) -> (String, i32, HashSet<String>) {
    let v: Vec<&str> = s.split(" -> ").collect();

    let name: String = re1.captures(v[0]).unwrap().get(1).unwrap().as_str().to_string();
    let weight: i32 = re1.captures(v[0]).unwrap().get(2).unwrap().as_str().parse().unwrap();
    let mut children = HashSet::new();

    if v.len() > 1 {
        for cap in re2.captures_iter(v[1]) {
            children.insert(cap.get(1).unwrap().as_str().to_string());
        }
    }

    (name, weight, children)
}

fn build_tree(name: String, data: &HashMap<String, HashSet<String>>, weights: &HashMap<String, i32>) -> Node {
    let mut children: Vec<Node> = vec![];

    for n in &data[&name] {
        let subtree = build_tree(n.to_string(), data, &weights);
        children.push(subtree);
    }

    let mut node = Node {
        name: String::new(),
        weight: weights.get(&name).unwrap().to_owned(),
        subtree: 0,
        children,
    };
    node.name = name;
    node
}

fn calc_tree(root: &mut Node) ->  {
}

fn main() {
    let re1 = Regex::new(r"(\w+) \((\d+)\)").unwrap();
    let re2 = Regex::new(r"(\w+)(?:, )?").unwrap();

    let mut items: HashMap<String, HashSet<String>> = HashMap::new();
    let mut weights: HashMap<String, i32> = HashMap::new();
    for line in io::stdin().lock().lines() {
        let p = parse(&line.unwrap(), &re1, &re2);
        items.insert(p.0.clone(), p.2);
        weights.insert(p.0.clone(), p.1);
    }

    let parents: HashSet<&String> = items.keys().collect();
    let mut children: HashSet<&String> = HashSet::new();
    for v in items.values() {
        for c in v {
            children.insert(c);
        }
    }

    let root: String = parents.difference(&children).into_iter().next().unwrap().to_string();
    println!("{:?}", root);

    // part 2
    let tree = build_tree(root, &items, &weights);
    println!("{:?}", tree);


}
