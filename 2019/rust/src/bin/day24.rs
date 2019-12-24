use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

const EX1: &str = "
....#
#..#.
#..##
..#..
#....";

fn parse(s: &str) -> Grid {
    s.trim().lines().map(|l| l.chars().collect()).collect()
}

fn adjacent(grid: &Grid, x: i32, y: i32) -> usize {
    let mut neighbors = vec![
        // (x - 1, y - 1),
        (x, y - 1),
        // (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        // (x - 1, y + 1),
        (x, y + 1),
        // (x + 1, y + 1),
    ];
    neighbors.retain(|&(x, y)| x >= 0 && x < 5 && y >= 0 && y < 5);

    let mut total = 0;
    for (x, y) in neighbors {
        if grid[y as usize][x as usize] == '#' {
            total += 1;
        }
    }
    total
}

fn evolve(prev: Grid) -> Grid {
    let mut next = Grid::new();
    for y in 0..5usize {
        let mut v = vec![];
        for x in 0..5usize {
            let adj = adjacent(&prev, x as i32, y as i32);
            let val = if prev[y][x] == '#' {
                if adj == 1 {
                    '#'
                } else {
                    '.'
                }
            } else {
                if adj == 1 || adj == 2 {
                    '#'
                } else {
                    '.'
                }
            };
            v.push(val);
        }
        next.push(v);
    }
    next
}

fn draw(grid: &Grid) {
    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn bio(grid: &Grid) -> u32 {
    let mut total = 0;
    let mut cur = 1;
    for line in grid {
        for c in line {
            if *c == '#' {
                total += cur;
            }
            cur *= 2;
        }
    }
    total
}

fn main() {
    let mut grid = parse(include_str!("../../../in/day24.in"));
    let mut seen = HashSet::new();

    loop {
        let score = bio(&grid);
        if seen.contains(&score) {
            println!("Part 1: {}", score);
            break;
        }

        seen.insert(score);
        grid = evolve(grid);
    }
}
