const INPUT: i32 = 5;

fn run_program(mut mem: Vec<i32>) -> Vec<i32> {
    let mut ip = 0;
    loop {
        if mem.len() > ip + 3 {
            println!(
                "{}: {} {} {} {}",
                ip,
                mem[ip],
                mem[ip + 1],
                mem[ip + 2],
                mem[ip + 3]
            );
        }

        match mem[ip] {
            1 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = mem[p1] + mem[p2];
                ip += 4;
            }
            101 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = p1 + mem[p2];
                ip += 4;
            }
            1001 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = mem[p1] + p2;
                ip += 4;
            }
            1101 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = p1 + p2;
                ip += 4;
            }
            2 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = mem[p1] * mem[p2];
                ip += 4;
            }
            102 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = p1 * mem[p2];
                ip += 4;
            }
            1002 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = mem[p1] * p2;
                ip += 4;
            }
            1102 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = p1 * p2;
                ip += 4;
            }
            3 => {
                let p1 = mem[ip + 1] as usize;
                mem[p1] = INPUT;
                ip += 2;
            }
            4 => {
                let p1 = mem[ip + 1] as usize;
                dbg!(mem[p1]);
                ip += 2;
            }
            104 => {
                let p1 = mem[ip + 1];
                dbg!(p1);
                ip += 2;
            }
            5 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                if mem[p1] != 0 {
                    ip = mem[p2] as usize;
                } else {
                    ip += 3;
                }
            }
            105 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2] as usize;
                if p1 != 0 {
                    ip = mem[p2] as usize;
                } else {
                    ip += 3;
                }
            }
            1005 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2];
                if mem[p1] != 0 {
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            }
            1105 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2];
                if p1 != 0 {
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                if mem[p1] == 0 {
                    ip = mem[p2] as usize;
                } else {
                    ip += 3;
                }
            }
            106 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2] as usize;
                if p1 == 0 {
                    ip = mem[p2] as usize;
                } else {
                    ip += 3;
                }
            }
            1006 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2];
                if mem[p1] == 0 {
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            }
            1106 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2];
                if p1 == 0 {
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            }
            // less than
            7 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if mem[p1] < mem[p2] { 1 } else { 0 };
                ip += 4;
            }
            107 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if p1 < mem[p2] { 1 } else { 0 };
                ip += 4;
            }
            1007 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if mem[p1] < p2 { 1 } else { 0 };
                ip += 4;
            }
            1107 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if p1 < p2 { 1 } else { 0 };
                ip += 4;
            }
            // equal
            8 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if mem[p1] == mem[p2] { 1 } else { 0 };
                ip += 4;
            }
            108 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2] as usize;
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if p1 == mem[p2] { 1 } else { 0 };
                ip += 4;
            }
            1008 => {
                let p1 = mem[ip + 1] as usize;
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if mem[p1] == p2 { 1 } else { 0 };
                ip += 4;
            }
            1108 => {
                let p1 = mem[ip + 1];
                let p2 = mem[ip + 2];
                let p3 = mem[ip + 3] as usize;
                mem[p3] = if p1 == p2 { 1 } else { 0 };
                ip += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
    mem
}

fn parse_program(s: &str) -> Vec<i32> {
    s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

fn main() {
    assert_eq!(vec![2, 0, 0, 0, 99], run_program(vec![1, 0, 0, 0, 99]));
    assert_eq!(vec![101, 0, 0, 0, 99], run_program(vec![101, 0, 0, 0, 99]));
    assert_eq!(vec![2, 3, 0, 6, 99], run_program(vec![2, 3, 0, 3, 99]));
    assert_eq!(
        vec![2, 4, 4, 5, 99, 9801],
        run_program(vec![2, 4, 4, 5, 99, 0])
    );
    assert_eq!(
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
    );

    // run test from day02
    let mut mem = parse_program(include_str!("../../../in/day02.in"));
    mem[1] = 76;
    mem[2] = 21;
    assert_eq!(19690720, run_program(mem)[0]);

    // day 5
    let mem = parse_program(include_str!("../../../in/day05.in"));
    run_program(mem);
}
