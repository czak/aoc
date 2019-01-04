use aoc2017::utils;

fn main() {
    let num: i32 = utils::read_number().unwrap();

    println!("Part 1: {}", part1(num));
    println!("Part 2: {}", part2(num));
}

fn part1(num: i32) -> i32 {
    let (layer, mut offset) = find_coords(num);
    offset = offset % (layer * 2);
    layer + (layer - offset - 1).abs()
}

fn find_coords(num: i32) -> (i32, i32) {
    let mut layer: i32 = 1;
    let mut start: i32 = 2;

    while start + layer * 8 <= num {
        start += layer * 8;
        layer += 1;
    }

    (layer, num - start)
}

fn radial_to_planar((layer, offset): (i32, i32)) -> (i32, i32) {
    let side = offset / (layer * 2);
    let shift = layer - offset % (layer * 2) - 1;
    match side {
        0 => (layer, shift),
        1 => (shift, -layer),
        2 => (-layer, -shift),
        3 => (-shift, layer),
        _ => panic!(),
    }
}

const SIZE: usize = 11;
const CENTER: usize = SIZE / 2;

fn print_grid(grid: &[[i32; SIZE]]) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{:8} ", cell);
        }
        println!()
    }
}

fn part2(num: i32) -> i32 {
    let mut grid: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    grid[CENTER][CENTER] = 1;
    for i in 2.. {
        let (x, y) = radial_to_planar(find_coords(i));
        let (x, y) = ((x + CENTER as i32) as usize, (y + CENTER as i32) as usize);

        let n = grid[y-1][x-1] + grid[y-1][x] + grid[y-1][x+1] +
                grid[y-0][x-1] +                grid[y-0][x+1] +
                grid[y+1][x-1] + grid[y+1][x] + grid[y+1][x+1];

        grid[y][x] = n;

        if n > num {
            print_grid(&grid);
            return n
        };
    }

    panic!()
}
