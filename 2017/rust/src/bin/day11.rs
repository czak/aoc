use std::io::{self, Read};
use std::ops::AddAssign;

#[derive(Debug)]
struct Point(i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Point {
    fn dist(&self) -> i32 {
        if self.0.signum() == self.1.signum() {
            self.0.abs() + self.1.abs()
        } else {
            std::cmp::max(self.0.abs(), self.1.abs())
        }
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut maxdist: i32 = 0;

    let mut p = Point(0, 0);
    for step in s.trim().split(",") {
        match step {
            "n" => p += Point(0, 1),
            "ne" => p += Point(1, 0),
            "se" => p += Point(1, -1),
            "s" => p += Point(0, -1),
            "sw" => p += Point(-1, 0),
            "nw" => p += Point(-1, 1),
            _ => panic!(),
        }
        if p.dist() > maxdist {
            maxdist = p.dist();
        }
    }

    println!("Part 1: {}", p.dist());
    println!("Part 2: {}", maxdist);
}
