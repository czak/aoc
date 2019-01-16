struct Generator {
    state: u64,
    factor: u64,
    divisor: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.state = (self.state * self.factor) % 2147483647;
            if self.state % self.divisor == 0 {
                break;
            }
        }
        Some(self.state)
    }
}

fn main() {
    // let gen_a = Generator { state: 65, factor: 16807, divisor: 4 };
    // let gen_b = Generator { state: 8921, factor: 48271, divisor: 8 };
    let gen_a = Generator { state: 289, factor: 16807, divisor: 4 };
    let gen_b = Generator { state: 629, factor: 48271, divisor: 8 };

    let mut count = 0;
    for (a, b) in gen_a.zip(gen_b).take(5000000) {
        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }

    println!("Part 2: {}", count);
}
