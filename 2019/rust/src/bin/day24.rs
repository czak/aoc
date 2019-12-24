use std::collections::HashSet;

const MARGIN: usize = 120;

// 0 1 2 3 4 [5] 6 7 8 9 10 (LEN = 11)
type World = Vec<Grid>;
type Grid = Vec<Vec<char>>;

#[allow(dead_code)]
const EX1: &str = "
....#
#..#.
#..##
..#..
#....";

const EMPTY: &str = "
.....
.....
.....
.....
.....";

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

//      |     |         |     |
//   1  |  2  |    3    |  4  |  5
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//   6  |  7  |    8    |  9  |  10
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |A|B|C|D|E|     |
//      |     |-+-+-+-+-|     |
//      |     |F|G|H|I|J|     |
//      |     |-+-+-+-+-|     |
//  11  | 12  |K|L|?|N|O|  14 |  15
//      |     |-+-+-+-+-|     |
//      |     |P|Q|R|S|T|     |
//      |     |-+-+-+-+-|     |
//      |     |U|V|W|X|Y|     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//  16  | 17  |    18   |  19 |  20
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//  21  | 22  |    23   |  24 |  25
//      |     |         |     |
fn adjacent2(world: &World, x: i32, y: i32, z: i32) -> usize {
    let mut neighbors = vec![(x, y - 1, z), (x - 1, y, z), (x + 1, y, z), (x, y + 1, z)];
    neighbors.retain(|&(x, y, _)| x >= 0 && x < 5 && y >= 0 && y < 5);
    // drop center,center (necessary?)
    neighbors.retain(|&(x, y, _)| (x, y) != (2, 2));
    // outside edge: add one level up
    if x == 0 {
        neighbors.push((1, 2, z - 1));
    }
    if x == 4 {
        neighbors.push((3, 2, z - 1));
    }
    if y == 0 {
        neighbors.push((2, 1, z - 1));
    }
    if y == 4 {
        neighbors.push((2, 3, z - 1));
    }

    // inside edge: add one level down
    if (x, y) == (1, 2) {
        neighbors.append(&mut vec![
            (0, 0, z + 1),
            (0, 1, z + 1),
            (0, 2, z + 1),
            (0, 3, z + 1),
            (0, 4, z + 1),
        ]);
    }
    if (x, y) == (3, 2) {
        neighbors.append(&mut vec![
            (4, 0, z + 1),
            (4, 1, z + 1),
            (4, 2, z + 1),
            (4, 3, z + 1),
            (4, 4, z + 1),
        ]);
    }
    if (x, y) == (2, 1) {
        neighbors.append(&mut vec![
            (0, 0, z + 1),
            (1, 0, z + 1),
            (2, 0, z + 1),
            (3, 0, z + 1),
            (4, 0, z + 1),
        ]);
    }
    if (x, y) == (2, 3) {
        neighbors.append(&mut vec![
            (0, 4, z + 1),
            (1, 4, z + 1),
            (2, 4, z + 1),
            (3, 4, z + 1),
            (4, 4, z + 1),
        ]);
    }

    neighbors.retain(|&(_, _, z)| z >= 0 && z <= 2 * MARGIN as i32);

    // println!("({},{},{}) => {:?}", x, y, z, neighbors);

    let mut total = 0;
    for (x, y, z) in neighbors {
        if world[z as usize][y as usize][x as usize] == '#' {
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

fn evolve2(prev: World) -> World {
    let mut next = World::new();
    for z in 0..prev.len() {
        let mut g = vec![];
        for y in 0..5usize {
            let mut v = vec![];
            for x in 0..5usize {
                let adj = adjacent2(&prev, x as i32, y as i32, z as i32);
                let val = if (x, y) == (2, 2) {
                    '.'
                } else if prev[z][y][x] == '#' {
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
            g.push(v);
        }
        next.push(g);
    }
    next
}

#[allow(dead_code)]
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

    let mut world = World::new();
    for _ in 0..MARGIN {
        world.push(parse(EMPTY));
    }
    world.push(parse(include_str!("../../../in/day24.in")));
    for _ in 0..MARGIN {
        world.push(parse(EMPTY));
    }

    // seconds
    for _ in 0..200 {
        world = evolve2(world);
    }

    let mut total = 0;

    for (_i, grid) in world.iter().enumerate() {
        let sum = grid.iter().flatten().filter(|&&c| c == '#').count();
        // println!("{}: {}", _i, sum);
        total += sum;
    }

    println!("Part 2: {}", total);
}
