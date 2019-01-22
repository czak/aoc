fn main() {
    let grid: Vec<Vec<char>> = include_str!("../../../in/day19.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut answer: Vec<char> = Vec::new();
    let mut count = 0;

    let mut x = grid[0].iter().position(|&c| c == '|').unwrap();
    let mut y = 0;
    let mut dx: isize = 0;
    let mut dy: isize = 1;

    loop {
        let c = grid[y][x];

        println!("{},{}: {}", x, y, c);
        
        if c == ' ' {
            break;
        } else if c == '+' {
            if dx != 0 {
                dx = 0;
                if y > 0 && grid[y-1][x] != ' ' {
                    dy = -1;
                } else {
                    dy = 1;
                }
            } else {
                dy = 0;
                if x > 0 && grid[y][x-1] != ' ' {
                    dx = -1;
                } else {
                    dx = 1;
                }
            }
        } else if c.is_alphabetic() {
            answer.push(c);
        }

        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
        count += 1;
    }

    println!("Part 1: {}", answer.iter().collect::<String>());
    println!("Part 2: {}", count);
}
