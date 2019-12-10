use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f32::consts::PI;

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

#[allow(dead_code)]
const EX4: &str = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";

macro_rules! fixed {
    ($f:expr) => {
        ($f * 1_000_000.0) as i32
    };
}

type Point = (i32, i32);
type Polar = (i32, i32);

fn polar(p: Point) -> Polar {
    (
        match (p.0 as f32).atan2(-p.1 as f32) {
            angle if angle >= 0.0 => fixed!(angle),
            angle => fixed!(2.0 * PI + (angle as f32)),
        },
        fixed!((p.0 as f32).hypot(p.1 as f32)),
    )
}

fn load(s: &str) -> Vec<Point> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn angles(origin: Point, targets: &[Point]) -> HashSet<i32> {
    targets
        .iter()
        .filter(|&&p| p != origin)
        .map(|&p| polar((p.0 - origin.0, p.1 - origin.1)).0)
        .collect()
}

fn part1(asteroids: &[Point]) -> (Point, usize) {
    asteroids
        .iter()
        .map(|&a| (a, angles(a, asteroids).len()))
        .max_by_key(|p| p.1)
        .unwrap()
}

fn part2(base: Point, asteroids: &[Point]) -> Vec<Point> {
    let mut m: HashMap<i32, BinaryHeap<(i32, Point)>> = HashMap::new();

    for &point in asteroids.iter().filter(|&&p| p != base) {
        let (angle, distance) = polar((point.0 - base.0, point.1 - base.1));
        let v = m.entry(angle).or_insert(BinaryHeap::new());
        v.push((-distance, point));
    }

    // sort by angle
    let mut angles: Vec<_> = m.keys().cloned().collect();
    angles.sort();

    let mut out: Vec<Point> = vec![];

    while m.values().any(|v| v.len() != 0) {
        for angle in &angles {
            if let Some((_distance, point)) = m.get_mut(&angle).unwrap().pop() {
                // println!("{}: {:?}", i, item);
                out.push(point);
                continue;
            }
        }
    }

    out
}

fn main() {
    let asteroids = load(include_str!("../../../in/day10.in"));

    // part 1

    assert_eq!(((3, 4), 8), part1(&load(EX1)));
    assert_eq!(((5, 8), 33), part1(&load(EX2)));
    assert_eq!(((11, 13), 210), part1(&load(EX3)));

    let (best, count) = part1(&asteroids);
    assert_eq!((19, 11), best);
    assert_eq!(230, count);

    println!(
        "Part 1: best is {:?} with {} asteroids detected",
        best, count
    );

    // part 2

    let _ex = part2((11, 13), &load(EX3));
    assert_eq!((11, 12), _ex[0]);
    assert_eq!((16, 9), _ex[49]);
    assert_eq!((8, 2), _ex[199]);

    let targets = part2((19, 11), &asteroids);
    assert_eq!((12, 5), targets[199]);

    println!("Part 2: {:?}", targets[199]);
}
