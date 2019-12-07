use itertools::Itertools;

#[derive(Clone)]
struct Intcode {
    mem: Vec<i32>,
    ip: usize,
}

impl Intcode {
    fn opcode(&self) -> i32 {
        self.mem[self.ip] % 100
    }

    fn arg(&self, num: usize) -> i32 {
        let mode = (self.mem[self.ip] / 10i32.pow(num as u32 + 1)) % 10;
        let val = self.mem[self.ip + num];
        if mode == 1 {
            val
        } else {
            self.mem[val as usize]
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

    fn resume(&mut self, input: Vec<i32>) -> Option<i32> {
        let mut it = input.iter();

        loop {
            // self.debug();

            match self.opcode() {
                1 => {
                    let p = self.mem[self.ip + 3] as usize;
                    self.mem[p] = self.arg(1) + self.arg(2);
                    self.ip += 4;
                }
                2 => {
                    let p = self.mem[self.ip + 3] as usize;
                    self.mem[p] = self.arg(1) * self.arg(2);
                    self.ip += 4;
                }
                3 => {
                    let p = self.mem[self.ip + 1] as usize;
                    self.mem[p] = *it.next().expect("Input not provided");
                    self.ip += 2;
                }
                4 => {
                    let output = self.arg(1);
                    self.ip += 2;
                    return Some(output);
                }
                5 => {
                    if self.arg(1) != 0 {
                        self.ip = self.arg(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    if self.arg(1) == 0 {
                        self.ip = self.arg(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let p = self.mem[self.ip + 3] as usize;
                    self.mem[p] = if self.arg(1) < self.arg(2) { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    let p = self.mem[self.ip + 3] as usize;
                    self.mem[p] = if self.arg(1) == self.arg(2) { 1 } else { 0 };
                    self.ip += 4;
                }
                99 => return None,
                _ => panic!("Invalid opcode"),
            }
        }
    }

    fn parse(s: &str) -> Self {
        Intcode {
            mem: s.trim().split(',').filter_map(|s| s.parse().ok()).collect(),
            ip: 0,
        }
    }
}

fn main() {
    // sanity check for Intcode machine
    let mut machine = Intcode::parse(include_str!("../../../in/day05.in"));
    assert_eq!(Some(584126), machine.resume(vec![5]));

    // day 07 part 2
    assert_eq!(
        139629729,
        thruster_signal(
            Intcode::parse(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            ),
            &[9, 8, 7, 6, 5]
        ),
    );

    assert_eq!(
        139629729,
        max_thruster(Intcode::parse(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        ))
    );

    let program = Intcode::parse(include_str!("../../../in/day07.in"));
    println!("Part 2: {}", max_thruster(program));
}

fn max_thruster(machine: Intcode) -> i32 {
    let mut max: i32 = -1_000_000;
    for sequence in (5..=9).permutations(5) {
        let signal = thruster_signal(machine.clone(), &sequence);
        if signal > max {
            // dbg!(sequence, signal);
            max = signal;
        }
    }
    max
}

fn thruster_signal(machine: Intcode, sequence: &[i32]) -> i32 {
    let mut m1 = machine.clone();
    let mut m2 = machine.clone();
    let mut m3 = machine.clone();
    let mut m4 = machine.clone();
    let mut m5 = machine.clone();

    // first run
    // ugly, expects no halt in first loop
    let mut o1 = m1.resume(vec![sequence[0], 0]);
    let mut o2 = m2.resume(vec![sequence[1], o1.unwrap()]);
    let mut o3 = m3.resume(vec![sequence[2], o2.unwrap()]);
    let mut o4 = m4.resume(vec![sequence[3], o3.unwrap()]);
    let mut o5 = m5.resume(vec![sequence[4], o4.unwrap()]);

    let mut last = o5.unwrap();

    loop {
        o1 = m1.resume(vec![o5.unwrap()]);
        if o1.is_none() {
            break;
        }
        o2 = m2.resume(vec![o1.unwrap()]);
        if o2.is_none() {
            break;
        }
        o3 = m3.resume(vec![o2.unwrap()]);
        if o3.is_none() {
            break;
        }
        o4 = m4.resume(vec![o3.unwrap()]);
        if o4.is_none() {
            break;
        }
        o5 = m5.resume(vec![o4.unwrap()]);
        if o5.is_none() {
            break;
        }

        last = o5.unwrap();

        // dbg!(last);
    }

    last
}
