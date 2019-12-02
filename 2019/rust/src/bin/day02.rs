use std::io::{self, Read};

fn run_program(mut mem: Vec<u32>) -> Vec<u32> {
    let mut ip = 0;
    loop {
        match mem[ip] {
            1 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = mem[p1] + mem[p2];
                ip += 4;
            }
            2 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = mem[p1] * mem[p2];
                ip += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
    mem
}

fn main() {
    assert_eq!(vec![2, 0, 0, 0, 99], run_program(vec![1, 0, 0, 0, 99]));
    assert_eq!(vec![2, 3, 0, 6, 99], run_program(vec![2, 3, 0, 3, 99]));
    assert_eq!(
        vec![2, 4, 4, 5, 99, 9801],
        run_program(vec![2, 4, 4, 5, 99, 0])
    );
    assert_eq!(
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
    );

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let program: Vec<u32> = s.trim().split(',').map(|s| s.parse().unwrap()).collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = program.clone();
            mem[1] = noun;
            mem[2] = verb;
            mem = run_program(mem);
            println!("noun={}, verb={}, out={}", noun, verb, mem[0]);

            if mem[0] == 19690720 {
                return;
            }
        }
    }
}
