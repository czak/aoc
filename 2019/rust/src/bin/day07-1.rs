use aoc2019::intcode;
use itertools::Itertools;

fn main() {
    assert_eq!(
        43210,
        thruster_signal(
            vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
            &[4, 3, 2, 1, 0]
        )
    );

    assert_eq!(
        43210,
        max_thruster(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ])
    );

    assert_eq!(
        54321,
        thruster_signal(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],
            &[0, 1, 2, 3, 4]
        )
    );

    assert_eq!(
        54321,
        max_thruster(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ])
    );

    let program = intcode::parse(include_str!("../../../in/day07.in"));
    println!("Part 1: {}", max_thruster(program));
}

fn max_thruster(program: Vec<i32>) -> i32 {
    let mut max: i32 = -1_000_000;
    for sequence in (0..=4).permutations(5) {
        let signal = thruster_signal(program.clone(), &sequence);
        if signal > max {
            // dbg!(sequence, signal);
            max = signal;
        }
    }
    max
}

fn thruster_signal(program: Vec<i32>, sequence: &[i32]) -> i32 {
    // TODO: too much cloning
    let o1 = intcode::run(program.clone(), vec![sequence[0], 0]);
    let o2 = intcode::run(program.clone(), vec![sequence[1], o1[0]]);
    let o3 = intcode::run(program.clone(), vec![sequence[2], o2[0]]);
    let o4 = intcode::run(program.clone(), vec![sequence[3], o3[0]]);
    let o5 = intcode::run(program.clone(), vec![sequence[4], o4[0]]);
    o5[0]
}
