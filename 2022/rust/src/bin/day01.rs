#[allow(dead_code)]
const EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

fn main() {
    // let input = EXAMPLE;
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|b| b.lines().fold(0, |sum, s| sum + s.parse::<u32>().unwrap()))
        .collect();
    elves.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", elves[0]);
    println!("Part 2: {}", elves.iter().take(3).sum::<u32>());
}
