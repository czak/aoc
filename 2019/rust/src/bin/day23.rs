#![allow(unused_imports)]
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time;

fn nic(address: i64, tx: mpsc::Sender<(usize, i64, i64)>, rx: mpsc::Receiver<(i64, i64)>) {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day23.in")),
        input: VecDeque::new(),
        rx,
        ip: 0,
        rb: 0,
    };

    ic.input.push_back(address);
    ic.mem.resize(2400, 0);

    while let (Some(target), Some(x), Some(y)) = (ic.next(), ic.next(), ic.next()) {
        tx.send((target as usize, x, y)).unwrap();
    }
}

fn main() {
    // tx do sklonowania, rx zostaje we switchu
    let (txs, rxs) = mpsc::channel::<(usize, i64, i64)>();

    let mut outputs = vec![];

    for i in 0..50 {
        let (tx, rx) = mpsc::channel::<(i64, i64)>();
        let txs = txs.clone();
        thread::spawn(move || nic(i, txs, rx));
        outputs.push(tx);
    }

    while let Ok((address, x, y)) = rxs.recv() {
        if address < outputs.len() {
            outputs[address].send((x, y)).unwrap();
        } else {
            if address == 255 {
                println!("Part 1: {}", y);
                break;
            }
        }
    }
}

mod intcode {
    use std::collections::VecDeque;
    use std::sync::mpsc;
    use std::time::Duration;

    pub struct Intcode {
        pub mem: Vec<i64>,
        pub ip: usize,
        pub rx: mpsc::Receiver<(i64, i64)>,
        pub input: VecDeque<i64>,
        pub rb: usize,
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

                // check input, pull into queue if any
                if let Ok((x, y)) = self.rx.try_recv() {
                    self.input.push_back(x);
                    self.input.push_back(y);
                }

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
                        let val = self.input.pop_front().unwrap_or(-1);
                        self.put(1, val);
                        self.ip += 2;
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
}
