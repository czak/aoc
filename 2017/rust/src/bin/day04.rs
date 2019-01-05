use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let mut lines: Vec<Vec<String>> = vec![];
    for line in io::stdin().lock().lines() {
        let line = line.unwrap()
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        lines.push(line);
    }

    println!("Part 1: {}", count_valid(&lines, is_valid));
    println!("Part 1: {}", count_valid(&lines, is_valid_anagram));
}

fn count_valid<P>(lines: &Vec<Vec<String>>, predicate: P) -> usize
where P: FnMut(&&Vec<String>) -> bool {
    lines.iter().filter(predicate).count()
}

fn is_valid(line: &&Vec<String>) -> bool {
    let s: HashSet<&String> = line.iter().collect();
    s.len() == line.len()
}

fn is_valid_anagram(line: &&Vec<String>) -> bool {
    let s: HashSet<String> = line.iter()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            chars.into_iter().collect()
        })
    .collect();
    s.len() == line.len()
}
