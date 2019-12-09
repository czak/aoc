#[derive(Clone, Default)]
struct Intcode {
    mem: Vec<i32>,
    ip: usize,
    input: Vec<i32>,
    inp: usize,
    rb: usize,
}

impl Intcode {
    fn opcode(&self) -> i32 {
        self.mem[self.ip] % 100
    }

    fn get(&self, arg: usize) -> i32 {
        let mode = (self.mem[self.ip] / 10i32.pow(arg as u32 + 1)) % 10;
        let val = self.mem[self.ip + arg];
        match mode {
            0 => self.mem[val as usize],
            1 => val,
            _ => panic!("Unknown mode for get"),
        }
    }

    fn put(&mut self, arg: usize, val: i32) {
        let mode = (self.mem[self.ip] / 10i32.pow(arg as u32 + 1)) % 10;
        let pos = self.mem[self.ip + arg] as usize;
        match mode {
            0 => self.mem[pos] = val,
            1 => panic!("Immediate not allowed for write"),
            _ => panic!("Unknown mode for put"),
        }
    }

    #[allow(dead_code)]
    fn debug(&self) {
        if self.mem.len() > self.ip + 3 {
            println!(
                "{}: {} {} {} {}",
                self.ip,
                self.mem[self.ip],
                self.mem[self.ip + 1],
                self.mem[self.ip + 2],
                self.mem[self.ip + 3]
            );
        }
    }
}

impl Iterator for Intcode {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.debug();

            match self.opcode() {
                1 => {
                    self.put(3, self.get(1) + self.get(2));
                    self.ip += 4;
                }
                2 => {
                    self.put(3, self.get(1) * self.get(2));
                    self.ip += 4;
                }
                3 => {
                    self.put(1, self.input[self.inp]);
                    self.ip += 2;
                    self.inp += 1;
                }
                4 => {
                    let output = self.get(1);
                    self.ip += 2;
                    return Some(output);
                }
                5 => {
                    if self.get(1) != 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    if self.get(1) == 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    self.put(3, if self.get(1) < self.get(2) { 1 } else { 0 });
                    self.ip += 4;
                }
                8 => {
                    self.put(3, if self.get(1) == self.get(2) { 1 } else { 0 });
                    self.ip += 4;
                }
                9 => {
                    self.rb += self.get(1) as usize;
                    self.ip += 2;
                }
                99 => return None,
                _ => panic!("Invalid opcode"),
            }
        }
    }
}

fn parse(s: &str) -> Vec<i32> {
    s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

fn main() {
    // sanity check for Intcode machine
    let intcode = Intcode {
        mem: parse(include_str!("../../../in/day05.in")),
        input: vec![5],
        ..Default::default()
    };
    assert_eq!(vec![584126], intcode.collect::<Vec<i32>>());
    //
    // // ex 1
    // let intcode = Intcode {
    //     mem: vec![
    //         109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    //     ],
    //     ..Default::default()
    // };
    // assert_eq!(
    //     vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,],
    //     intcode.collect::<Vec<i32>>()
    // );
}
