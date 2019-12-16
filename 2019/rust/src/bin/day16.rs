const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

struct Pattern {
    level: usize,
    i: usize,
}

impl Pattern {
    fn for_level(level: usize) -> Pattern {
        Pattern { level, i: 1 }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = (self.i / (self.level + 1)) % BASE_PATTERN.len();
        self.i += 1;
        Some(BASE_PATTERN[offset])
    }
}

fn fft(signal: &[i32]) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    for i in 0..signal.len() {
        let pat = Pattern::for_level(i);
        output.push(
            signal
                .iter()
                .zip(pat)
                .fold(0, |acc, (a, b)| acc + a * b)
                .abs()
                % 10,
        );
    }
    output
}

fn main() {
    assert_eq!(
        vec![1, 0, -1, 0, 1],
        Pattern::for_level(0).take(5).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1],
        Pattern::for_level(1).take(10).collect::<Vec<_>>()
    );

    // parsing
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], parse("12345678"));

    // fft
    assert_eq!(
        vec![3, 4, 0, 4, 0, 4, 3, 8],
        fft(&fft(&vec![1, 2, 3, 4, 5, 6, 7, 8]))
    );

    // part 1 examples
    assert_eq!(
        parse("24176176"),
        part1(parse("80871224585914546619083218645595"))
    );

    println!(
        "Part 1: {:?}",
        part1(parse(include_str!("../../../in/day16.in")))
    );
}

fn parse(s: &str) -> Vec<i32> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn part1(input: Vec<i32>) -> Vec<i32> {
    std::iter::successors(Some(input), |i| Some(fft(i)))
        .skip(100)
        .next()
        .unwrap()
        .iter()
        .copied()
        .take(8)
        .collect()
}
