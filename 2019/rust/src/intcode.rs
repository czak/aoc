pub fn parse(s: &str) -> Vec<i32> {
    s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

pub fn run(mut mem: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    let mut ip = 0; // instruction pointer
    let mut inp = 0; // input pointer

    let mut output: Vec<i32> = Vec::new();

    fn parse_opcode(opcode: i32) -> (bool, bool, bool, i32) {
        (
            ((opcode / 100) % 10 == 1),
            ((opcode / 1000) % 10 == 1),
            ((opcode / 10000) % 10 == 1),
            opcode % 100,
        )
    }

    fn arg(mem: &Vec<i32>, p: usize, m: bool) -> i32 {
        let val = mem[p];
        if m {
            val
        } else {
            mem[val as usize]
        }
    }

    loop {
        // if mem.len() > ip + 3 {
        //     println!(
        //         "{}: {} {} {} {}",
        //         ip,
        //         mem[ip],
        //         mem[ip + 1],
        //         mem[ip + 2],
        //         mem[ip + 3]
        //     );
        // }
        //
        let (m1, m2, m3, op) = parse_opcode(mem[ip]);

        match op {
            1 => {
                let p = mem[ip + 3] as usize;
                mem[p] = arg(&mem, ip + 1, m1) + arg(&mem, ip + 2, m2);
                ip += 4;
            }
            2 => {
                let p = mem[ip + 3] as usize;
                mem[p] = arg(&mem, ip + 1, m1) * arg(&mem, ip + 2, m2);
                ip += 4;
            }
            3 => {
                let p1 = mem[ip + 1] as usize;
                mem[p1] = input[inp];
                ip += 2;
                inp += 1;
            }
            4 => {
                output.push(arg(&mem, ip + 1, m1));
                ip += 2;
            }
            5 => {
                if arg(&mem, ip + 1, m1) != 0 {
                    ip = arg(&mem, ip + 2, m2) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                if arg(&mem, ip + 1, m1) == 0 {
                    ip = arg(&mem, ip + 2, m2) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let p = mem[ip + 3] as usize;
                mem[p] = if arg(&mem, ip + 1, m1) < arg(&mem, ip + 2, m2) {
                    1
                } else {
                    0
                };
                ip += 4;
            }
            8 => {
                let p = mem[ip + 3] as usize;
                mem[p] = if arg(&mem, ip + 1, m1) == arg(&mem, ip + 2, m2) {
                    1
                } else {
                    0
                };
                ip += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
    output
}
