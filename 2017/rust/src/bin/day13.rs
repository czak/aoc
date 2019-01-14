use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
struct Scanner {
    range: i32,
    pos: i32,
    dir: i32,
}

#[allow(dead_code)]
fn print_layers(layers: &HashMap<i32, Scanner>) {
    let max_depth: i32 = *layers.keys().max().unwrap();
    for i in 0..=max_depth {
        print!("{}: ", i);
        if let Some(Scanner { range, pos, .. }) = layers.get(&i) {
            for j in 0..*range {
                if j == *pos {
                    print!("[S] ");
                } else {
                    print!("[ ] ");
                }
            }
        } else {
            print!("...");
        }
        println!();
    }
}

fn step(scanner: &mut Scanner) {
    let next_pos = scanner.pos + scanner.dir;
    if next_pos < 0 || next_pos >= scanner.range {
        scanner.dir = -scanner.dir;
    }
    scanner.pos += scanner.dir;
}

fn step_back(scanner: &mut Scanner) {
    let next_pos = scanner.pos - scanner.dir;
    if next_pos < 0 || next_pos >= scanner.range {
        scanner.dir = -scanner.dir;
    }
    scanner.pos -= scanner.dir;
}

fn reset(scanner: &mut Scanner) {
    scanner.pos = 0;
    scanner.dir = 1;
}

fn severity(layers: &mut HashMap<i32, Scanner>, delay: i32) -> Option<i32> {
    let max_depth: i32 = *layers.keys().max().unwrap();
    let mut severity: i32 = 0;
    let mut caught = false;

    layers.values_mut().for_each(reset);

    for _ in 0..delay {
        layers.values_mut().for_each(step);
    }

    for i in 0..=max_depth {
        // println!("Picosecond {}, pos {}", i+delay, i);
        // print_layers(&layers);

        // entering layer i, check if scanner at pos 0
        if let Some(Scanner { range, pos: 0, .. }) = layers.get(&i) {
            // println!("Caught at layer {}, severity: {}", i, i*range);
            severity += i * range;
            caught = true;
        }

        // println!("---------------");

        layers.values_mut().for_each(step);
    }

    if caught {
        Some(severity)
    } else {
        None
    }
}

fn caught(layers: &mut HashMap<i32, Scanner>) -> bool {
    let max_depth: i32 = *layers.keys().max().unwrap();
    let mut caught = false;

    for i in 0..=max_depth {
        if let Some(Scanner { pos: 0, .. }) = layers.get(&i) {
            caught = true;
        }

        layers.values_mut().for_each(step);
    }

    for _ in 0..=max_depth {
        layers.values_mut().for_each(step_back);
    }

    caught
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut layers: HashMap<i32, Scanner> = HashMap::new();
    for line in input.lines() {
        let numbers: Vec<i32> = line.split(": ").map(|s| s.parse().unwrap()).collect();
        let scanner = Scanner {
            range: numbers[1],
            pos: 0,
            dir: 1,
        };
        layers.insert(numbers[0], scanner);
    }

    println!("Part 1: {}", severity(&mut layers, 0).unwrap());

    layers.values_mut().for_each(reset);

    for delay in 0.. {
        if delay % 100000 == 0 {
            println!("Checking {}...", delay);
        }
        if !caught(&mut layers) {
            println!("Part 2: {}", delay);
            break;
        }
        layers.values_mut().for_each(step);
    }
}
