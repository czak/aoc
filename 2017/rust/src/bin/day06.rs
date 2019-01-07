use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut banks: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    seen.insert(banks.clone());
    let mut vseen: Vec<Vec<i32>> = Vec::new();
    vseen.push(banks.clone());

    let mut cycles = 0;
    loop {
        let mut max: i32 = *banks.iter().max().unwrap();
        let mut pos: usize = banks.iter().position(|&e| e == max).unwrap();
        banks[pos] = 0;

        while max > 0 {
            pos = (pos + 1) % banks.len();
            banks[pos] += 1;
            max -= 1;
        }

        cycles += 1;
        println!("After {}: {:?}", cycles, banks);

        if seen.contains(&banks) {
            let idx = vseen.iter().position(|e| e == &banks).unwrap();
            println!("{}, {}", cycles, vseen.len() - idx);
            break;
        }

        seen.insert(banks.clone());
        vseen.push(banks.clone());
    }
}
