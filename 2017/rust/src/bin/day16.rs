use std::io::{self, Read};
use std::str::FromStr;
use std::collections::VecDeque;
use std::iter::Iterator;

#[derive(Debug)]
enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('s') => Ok(Step::Spin(s[1..].parse::<usize>().unwrap())),
            Some('x') => {
                let positions: Vec<usize> = s[1..].split('/').map(|s| s.parse().unwrap()).collect();
                Ok(Step::Exchange(positions[0], positions[1]))
            },
            Some('p') => {
                let n1: char = s.as_bytes()[1] as char;
                let n2: char = s.as_bytes()[3] as char;
                Ok(Step::Partner(n1, n2))
            },
            _ => Err(()),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // let input = "s1,x3/4,pe/b";

    let mut programs: VecDeque<char> =
        (('a' as u8)..=('p' as u8)).map(|c| c as char).collect();

    let steps: Vec<Step> = input.split(",").map(|s| s.parse().unwrap()).collect();

    for i in 1.. {
        for s in steps.iter() {
            match *s {
                Step::Spin(count) => {
                    for _ in 0..count {
                        let p = programs.pop_back().unwrap();
                        programs.push_front(p);
                    }
                },
                Step::Exchange(a, b) => {
                    programs.swap(a, b);
                },
                Step::Partner(p, q) => {
                    let a = programs.iter().position(|&c| c == p).unwrap();
                    let b = programs.iter().position(|&c| c == q).unwrap();
                    programs.swap(a, b);
                },
            }
        }

        let s = programs.iter().collect::<String>();
        if s.starts_with("knmdfoij") {
            println!("{}: {} ----- ", i, s);
        } else {
            println!("{}: {}", i, s);
        }
    }
}
