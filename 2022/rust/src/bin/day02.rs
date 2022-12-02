use std::collections::HashMap;

#[allow(dead_code)]
const EXAMPLE: &str = "A Y\nB X\nC Z";

fn main() {
    // let input = EXAMPLE;
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    let score_map = HashMap::from([
        ("A X", 1 + 3),
        ("B X", 1 + 0),
        ("C X", 1 + 6),
        ("A Y", 2 + 6),
        ("B Y", 2 + 3),
        ("C Y", 2 + 0),
        ("A Z", 3 + 0),
        ("B Z", 3 + 6),
        ("C Z", 3 + 3),
    ]);

    let select_map = HashMap::from([
        ("A X", "A Z"),
        ("B X", "B X"),
        ("C X", "C Y"),
        ("A Y", "A X"),
        ("B Y", "B Y"),
        ("C Y", "C Z"),
        ("A Z", "A Y"),
        ("B Z", "B Z"),
        ("C Z", "C X"),
    ]);

    let p1: u32 = input.lines().map(|l| score_map[l]).sum();
    let p2: u32 = input.lines().map(|l| score_map[select_map[l]]).sum();

    dbg!(p1);
    dbg!(p2);
}
