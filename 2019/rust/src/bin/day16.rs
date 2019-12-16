const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

struct Pattern {
    level: usize,
    i: usize,
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = (self.i / (self.level + 1)) % 4;
        self.i += 1;
        Some(BASE_PATTERN[offset])
    }
}

fn main() {
    assert_eq!(
        vec![0, 1, 0, -1],
        Pattern { level: 0, i: 0 }.take(4).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![0, 0, 1, 1, 0, 0, -1, -1, 0, 0],
        Pattern { level: 1, i: 0 }.take(10).collect::<Vec<_>>()
    );
}
