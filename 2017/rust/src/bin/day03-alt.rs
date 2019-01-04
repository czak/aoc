struct Navigator {
    x: i32,
    y: i32,
    i: i32,
    j: i32,
}

impl Navigator {
    fn new() -> Navigator {
        Navigator {
            x: 0,
            y: 0,
            i: 0,
            j: 0,
        }
    }
}

impl Iterator for Navigator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let pos = Some((self.x, self.y));

        let stride = (self.i + 2) / 2;
        let dir = self.i % 4;

        match dir {
            0 => self.x += 1,
            1 => self.y -= 1,
            2 => self.x -= 1,
            3 => self.y += 1,
            _ => panic!(),
        };

        self.j += 1;
        if self.j == stride {
            self.i += 1;
            self.j = 0;
        }

        pos
    }
}

fn main() {
    let num = aoc2017::utils::read_number().unwrap();

    println!("Part 1: {}", part1(num));
    println!("Part 2: {}", part2(num));

}

fn part1(num: i32) -> i32 {
    let mut nav = Navigator::new().skip((num-1) as usize);
    let (x, y) = nav.next().unwrap();
    x.abs() + y.abs()
}

fn part2(num: i32) -> i32 {
    use std::collections::HashMap;

    let mut grid = HashMap::new();
    let mut nav = Navigator::new();
    grid.insert(nav.next().unwrap(), 1);

    loop {
        let (x, y) = nav.next().unwrap();
        let mut i = 0;
        for cell in &[(x-1, y-1), (x-1, y), (x-1, y+1), (x-0, y-1), (x-0, y+1), (x+1, y-1), (x+1, y), (x+1, y+1)] {
            i += grid.get(cell).unwrap_or(&0);
        }
        grid.insert((x, y), i);
        if i > num {
            return i
        }
    }
}
