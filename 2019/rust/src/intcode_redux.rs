#[derive(Clone, Default)]
struct Intcode {
    mem: Vec<i64>,
    ip: usize,
    input: Vec<i64>,
    inp: usize,
    rb: usize,
}

impl Intcode {
    fn opcode(&self) -> i64 {
        self.mem[self.ip] % 100
    }

    fn get(&self, arg: usize) -> i64 {
        let mode = (self.mem[self.ip] / 10i64.pow(arg as u32 + 1)) % 10;
        let val = self.mem[self.ip + arg];
        match mode {
            0 => self.mem[val as usize],
            1 => val,
            2 => self.mem[(self.rb as i64 + val) as usize],
            _ => panic!("Unknown mode for get"),
        }
    }

    fn put(&mut self, arg: usize, val: i64) {
        let mode = (self.mem[self.ip] / 10i64.pow(arg as u32 + 1)) % 10;
        let pos = self.mem[self.ip + arg];
        match mode {
            0 => self.mem[pos as usize] = val,
            1 => panic!("Immediate not allowed for write"),
            2 => self.mem[(self.rb as i64 + pos) as usize] = val,
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
            println!("ip={}, rb={}", self.ip, self.rb);
        }
    }
}

impl Iterator for Intcode {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // self.debug();

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
                    self.rb = (self.rb as i64 + self.get(1)) as usize;
                    self.ip += 2;
                }
                99 => return None,
                _ => panic!("Invalid opcode"),
            }
        }
    }
}

pub fn parse(s: &str) -> Vec<i64> {
    s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

#[test]
fn test_intcode() {
    // sanity check for Intcode machine
    let intcode = Intcode {
        mem: parse(include_str!("../../in/day05.in")),
        input: vec![5],
        ..Default::default()
    };
    assert_eq!(vec![584126], intcode.collect::<Vec<i64>>());
}
