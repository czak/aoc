use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Step {
    type Err = String;

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
            _ => panic!(),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for s in input.split(",") {
        println!("{:?}", Step::from_str(s));
    }
}
