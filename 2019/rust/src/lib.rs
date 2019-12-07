pub mod intcode {
    pub fn parse(s: &str) -> Vec<i32> {
        s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
    }

    pub fn run(mut mem: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
        let mut ip = 0; // instruction pointer
        let mut inp = 0; // input pointer

        let mut output: Vec<i32> = Vec::new();

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
                    mem[p1] = input[inp];
                    ip += 2;
                    inp += 1;
                }
                4 => {
                    let p1 = mem[ip + 1] as usize;
                    output.push(mem[p1]);
                    ip += 2;
                }
                104 => {
                    let p1 = mem[ip + 1];
                    output.push(p1);
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
        output
    }
}
