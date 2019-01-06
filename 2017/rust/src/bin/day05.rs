use std::io;
use std::io::prelude::*;

fn main() {
    let mut jumps: Vec<i32> = vec![];
    for line in io::stdin().lock().lines() {
        jumps.push(line.unwrap().parse().unwrap());
    }

    let mut steps = 0;
    let mut idx: i32 = 0;
    while idx >= 0 && idx < jumps.len() as i32 {
        let instruction = &mut jumps[idx as usize];
        let offset = *instruction;
        if offset >= 3 {
            *instruction -= 1;
        } else {
            *instruction += 1;
        }
        idx += offset;
        steps += 1;
    }

    println!("{}", steps);
}
