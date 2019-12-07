use aoc2019::intcode;

fn main() {
    // day 5
    let mem = intcode::parse(include_str!("../../../in/day05.in"));

    assert_eq!(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 13210611],
        intcode::run(mem.clone(), vec![1])
    );

    assert_eq!(vec![584126], intcode::run(mem.clone(), vec![5]));
}
