struct Buffer {
    data: Vec<u32>,
    current: usize,
}

impl Buffer {
    fn skip(&mut self, count: usize) {
        self.current = (self.current + count) % self.data.len();
    }

    fn insert(&mut self, value: u32) {
        self.data.insert(self.current + 1, value);
        self.current += 1;
    }
}

fn main() {
    let step = 367;

    let mut buffer = Buffer {
        data: vec![0],
        current: 0,
    };

    for i in 1..=2017 {
        buffer.skip(step);
        buffer.insert(i);
    }

    println!("Part 1: {}", buffer.data[buffer.current+1]);

    let mut current = 0;
    let mut res = 0;

    for i in 1..=50000000 {
        current = (current + step) % i + 1;

        if current == 1 {
            res = i;
        }
    }

    println!("Part 2: {}", res);
}
