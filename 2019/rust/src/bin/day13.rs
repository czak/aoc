use aoc2019::intcode_redux as intcode;
use std::collections::HashSet;

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day13-hacked.in")),
        input: vec![0; 5000],
        ..Default::default()
    };
    ic.mem.resize(10000, 0);

    println!("Part 1: {}", part1(ic.clone()));

    ic.mem[0] = 2;

    println!("Part 2: {}", part2(ic));
}

fn part1(ic: intcode::Intcode) -> usize {
    let output: Vec<i64> = ic.collect();
    let mut blocks: HashSet<(i64, i64)> = HashSet::new();

    for chunk in output.chunks(3) {
        let x = chunk[0];
        let y = chunk[1];
        let id = chunk[2];

        if id == 2 {
            blocks.insert((x, y));
        } else {
            blocks.remove(&(x, y));
        }
    }

    blocks.len()
}

const WIDTH: usize = 41;
const HEIGHT: usize = 25;

type Board = [[i64; WIDTH]; HEIGHT];

fn part2(mut ic: intcode::Intcode) -> i64 {
    let mut board = [[0; WIDTH]; HEIGHT];
    let mut score: i64 = 0;

    while let (Some(x), Some(y), Some(id)) = (ic.next(), ic.next(), ic.next()) {
        if x >= 0 {
            board[y as usize][x as usize] = id;
        } else {
            score = id;
        }

        // draw(&board);
    }

    score
}

#[allow(dead_code)]
fn draw(board: &Board) {
    for line in board.iter() {
        for tile in line.iter() {
            match tile {
                0 => print!(" "),
                1 => print!("#"),
                2 => print!("$"),
                3 => print!("-"),
                4 => print!("o"),
                _ => panic!(),
            }
        }
        println!();
    }
}
