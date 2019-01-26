use std::collections::HashMap;

const MAP4: [[usize; 4]; 8] = [
    [0, 1, 2, 3],
    [2, 0, 3, 1],
    [3, 2, 1, 0],
    [1, 3, 0, 2],
    [1, 0, 3, 2],
    [3, 1, 2, 0],
    [2, 3, 0, 1],
    [0, 2, 1, 3],
];

const MAP9: [[usize; 9]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [6, 3, 0, 7, 4, 1, 8, 5, 2],
    [8, 7, 6, 5, 4, 3, 2, 1, 0],
    [2, 5, 8, 1, 4, 7, 0, 3, 6],
    [2, 1, 0, 5, 4, 3, 8, 7, 6],
    [8, 5, 2, 7, 4, 1, 6, 3, 0],
    [6, 7, 8, 3, 4, 5, 0, 1, 2],
    [0, 3, 6, 1, 4, 7, 2, 5, 8],
];


fn convert(block: &Vec<u32>, map: &HashMap<Vec<u32>, Vec<u32>>) -> Vec<u32> {
    if map.contains_key(block) {
        // dbg!(block);
        map[block].clone()
    } else {
        // dbg!(block);
        panic!()
    }
}

fn parse(s: &str) -> Vec<u32> {
    s.chars().filter_map(|c| {
        match c {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        }
    }).collect()
}

fn rotated(v: &Vec<u32>, variant: usize) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];

    if v.len() == 4 {
        for &i in &MAP4[variant] {
            res.push(v[i]);
        }
    } else if v.len() == 9 {
        for &i in &MAP9[variant] {
            res.push(v[i]);
        }
    } else {
        panic!();
    }

    res
}

fn read_input() -> std::collections::HashMap<Vec<u32>, Vec<u32>> {
    use std::io::BufRead;

    let mut map = std::collections::HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" => ").collect();
        let from = parse(parts[0]);
        let to = parse(parts[1]);

        for variant in 0..8 {
            map.insert(rotated(&from, variant), to.clone());
        }
    }

    map
}

fn main() {
    let mut grid: Vec<u32> = vec![
        0, 1, 0,
        0, 0, 1,
        1, 1, 1,
    ];

    let map = read_input();

    for _iter in 0..18 {
        let size = (grid.len() as f32).sqrt() as usize;
        let step;
        let new_size;
        let new_step;
        if size % 2 == 0 {
            step = 2;
            new_step = 3;
            new_size = size * 3 / 2;
        } else {
            step = 3;
            new_step = 4;
            new_size = size * 4 / 3;
        }

        let mut res: Vec<u32> = vec![0; new_size*new_size];

        for y in 0..size/step {
            for x in 0..size/step {
                let mut block = vec![];
                for j in 0..step {
                    for i in 0..step {
                        let v = grid[y*size*step + j*size + x*step + i];
                        block.push(v);
                    }
                }
                let mut new_block = convert(&block, &map);

                for j in (0..new_step).rev() {
                    for i in (0..new_step).rev() {
                        let v = new_block.pop().unwrap();
                        res[y*new_size*new_step + j*new_size + x*new_step + i] = v;
                    }
                }
            }
        }

        grid = res;
    }

    println!("Part 2: {}", grid.iter().sum::<u32>());
}
