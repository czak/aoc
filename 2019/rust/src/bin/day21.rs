use aoc2019::intcode_redux as intcode;

// J := (!1 | !2 | !3) & 4
const PROGRAM1: &str = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

// J := (!1 | !2 | !3) & 4 & ((!5 & 8) | 5)
const PROGRAM2: &str = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
AND H T
OR E T
AND T J
RUN
";

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day21.in")),
        input: PROGRAM1.chars().map(|c| c as i64).collect(),
        ..Default::default()
    };
    ic.mem.resize(5000, 0);

    println!("Part 1: {}", ic.last().unwrap());

    // Part 2
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day21.in")),
        input: PROGRAM2.chars().map(|c| c as i64).collect(),
        ..Default::default()
    };
    ic.mem.resize(5000, 0);

    println!("Part 2: {}", ic.last().unwrap());
}
