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

fn main() {
    assert_eq!(
        vec![1, 0, -1, 0, 1],
        Pattern::for_level(0).take(5).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1],
        Pattern::for_level(1).take(10).collect::<Vec<_>>()
    );
}
