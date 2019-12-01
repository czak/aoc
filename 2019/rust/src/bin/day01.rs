use std::io::{self, BufRead};

fn fuel_requirement(mass: i32) -> i32 {
    (mass as f32 / 3.0).floor() as i32 - 2
}

fn total_requirement(mass: i32) -> i32 {
    let req = fuel_requirement(mass);
    if req <= 0 {
        0
    } else {
        req + total_requirement(req)
    }
}

fn main() {
    let masses: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    // Part 1

    assert_eq!(2, fuel_requirement(12));
    assert_eq!(2, fuel_requirement(14));
    assert_eq!(654, fuel_requirement(1969));
    assert_eq!(33583, fuel_requirement(100756));

    let total: i32 = masses.iter().map(|&m| fuel_requirement(m)).sum();
    println!("Part 1: {}", total);

    // Part 2

    assert_eq!(2, total_requirement(14));
    assert_eq!(966, total_requirement(1969));
    assert_eq!(50346, total_requirement(100756));

    let total: i32 = masses.iter().map(|&m| total_requirement(m)).sum();
    println!("Part 2: {}", total);
}
