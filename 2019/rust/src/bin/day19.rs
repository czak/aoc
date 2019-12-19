#![allow(unused_imports)]
use aoc2019::intcode_redux as intcode;

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day19.in")),
        ..Default::default()
    };
    ic.mem.resize(500, 0);

    let mut total = 0;

    for y in 0..50 {
        for x in 0..50 {
            let mut ic = ic.clone();
            ic.input.push(x);
            ic.input.push(y);

            total += ic.next().unwrap();
        }
    }

    println!("Part 1: {}", total);

    for y in 850..1500 {
        let mut lights = 0;
        let mut first = -1;

        for x in 450..1000 {
            let mut ic = ic.clone();

            ic.input.push(x);
            ic.input.push(y);

            if let Some(1) = ic.next() {
                if first == -1 {
                    first = x;
                }
                lights += 1;
            }
        }

        // shift
        let offset = first + lights - 100;

        // spr czy 100 w dół też jest 1
        let mut ic = ic.clone();
        ic.input.push(offset);
        ic.input.push(y + 99);

        if let Some(1) = ic.next() {
            println!("Part 2: {}", offset * 10_000 + y);
            break;
        }
    }
}
