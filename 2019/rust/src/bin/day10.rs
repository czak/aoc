use std::collections::HashSet;

#[allow(dead_code)]
const EX1: &str = ".#..#
.....
#####
....#
...##";

#[allow(dead_code)]
const EX2: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

#[allow(dead_code)]
const EX3: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

type Point = (f64, f64);

fn load(s: &str) -> Vec<Point> {
    let mut res = vec![];
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                res.push((x as f64, y as f64));
            }
        }
    }
    res
}

fn direction(origin: Point, target: Point) -> (i64, i64) {
    if origin == target {
        return (0, 0);
    }
    let dir = (target.0 - origin.0, target.1 - origin.1);
    let len = dir.0.hypot(dir.1);
    (
        (dir.0 / len * 10_000_000.0).round() as i64,
        (dir.1 / len * 10_000_000.0).round() as i64,
    )
}

fn directions(origin: Point, targets: &Vec<Point>) -> HashSet<(i64, i64)> {
    targets
        .iter()
        .map(|&t| direction(origin, t))
        .filter(|&d| d != (0, 0))
        .collect()
}

fn main() {
    let asteroids = load(include_str!("../../../in/day10.in"));

    let best = asteroids
        .iter()
        .cloned()
        .max_by_key(|&a| directions(a, &asteroids).len())
        .unwrap();

    println!(
        "Part 1: best is {:?} with {} asteroids detected",
        best,
        directions(best, &asteroids).len()
    );
}
