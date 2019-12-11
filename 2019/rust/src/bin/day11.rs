use std::collections::HashMap;

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

type Point = (i32, i32);
type Color = i64;

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }
}

fn main() {
    test_intcode();

    let mut grid: [[i64; 43]; 6] = [[0; 43]; 6];
    grid[0][0] = 1;

    let mut visited: HashMap<Point, Color> = HashMap::new();
    visited.insert((0, 0), 1);

    let mut current: Point = (0, 0);
    let mut direction = Direction::Up;

    let mut mem = parse(include_str!("../../../in/day11.in"));
    mem.resize(1200, 0);

    let mut intcode = Intcode {
        mem,
        input: vec![1],
        ..Default::default()
    };

    while let Some(color) = intcode.next() {
        visited.insert(current, color);
        grid[current.1 as usize][current.0 as usize] = color;

        // new direction
        direction = match intcode.next().unwrap() {
            0 => direction.turn_left(),
            1 => direction.turn_right(),
            _ => panic!("Unknown direction"),
        };

        // move
        current = match direction {
            Direction::Left => (current.0 - 1, current.1),
            Direction::Up => (current.0, current.1 - 1),
            Direction::Right => (current.0 + 1, current.1),
            Direction::Down => (current.0, current.1 + 1),
        };

        // provide current color as input
        let current_color: Color = *visited.get(&current).unwrap_or(&0);
        intcode.input.push(current_color);
    }

    println!("Part 2:");
    for line in grid.iter() {
        for color in line.iter() {
            if *color == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn parse(s: &str) -> Vec<i64> {
    s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

fn test_intcode() {
    // sanity check for Intcode machine
    let intcode = Intcode {
        mem: parse(include_str!("../../../in/day05.in")),
        input: vec![5],
        ..Default::default()
    };
    assert_eq!(vec![584126], intcode.collect::<Vec<i64>>());
}
