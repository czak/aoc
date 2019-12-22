#![allow(dead_code)]
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Shuffle {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(i32),
}

impl FromStr for Shuffle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            return Ok(Shuffle::DealIntoNewStack);
        }
        let n: i32 = s.rsplit(' ').next().unwrap().parse().unwrap();
        if s.starts_with("cut") {
            Ok(Shuffle::Cut(n))
        } else {
            Ok(Shuffle::DealWithIncrement(n))
        }
    }
}

fn main() {
    // Part 1

    let seq = parse(include_str!("../../../in/day22.in"));
    println!("Part 1: {}", reindex_seq(2019, 10007, &seq));
    assert_eq!(2519, reindex_seq(2019, 10007, &seq));
}

fn parse(s: &str) -> Vec<Shuffle> {
    s.trim().lines().map(|l| l.parse().unwrap()).collect()
}

const EX1: &str = "
deal with increment 7
deal into new stack
deal into new stack
";

const EX2: &str = "
cut 6
deal with increment 7
deal into new stack
";

const EX3: &str = "
deal with increment 7
deal with increment 9
cut -2";

const EX4: &str = "
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

fn reindex(i: i32, size: i32, shuffle: Shuffle) -> i32 {
    use Shuffle::*;
    match shuffle {
        DealIntoNewStack => (size - 1 - i) % size,
        Cut(n) => {
            let split = if n >= 0 { n } else { size + n };
            (i + size - split) % size
        }
        DealWithIncrement(n) => (n * i) % size,
    }
}

#[test]
fn test_reindexing() {
    assert_eq!(6, reindex(3, 10, Shuffle::DealIntoNewStack));
    assert_eq!(
        3,
        reindex(
            reindex(3, 10, Shuffle::DealIntoNewStack),
            10,
            Shuffle::DealIntoNewStack
        )
    );

    // 0 1 2 3 4 5 6 7 8 9
    // 3 4 5 6 7 8 9 0 1 2
    assert_eq!(8, reindex(1, 10, Shuffle::Cut(3)));

    // 0 1 2 3 4 5 6 7 8 9
    // 6 7 8 9 0 1 2 3 4 5
    assert_eq!(5, reindex(1, 10, Shuffle::Cut(-4)));
    assert_eq!(2, reindex(8, 10, Shuffle::Cut(-4)));

    // 0 1 2 3 4 5 6 7 8 9
    // 0 7 4 1 8 5 2 9 6 3
    assert_eq!(3, reindex(1, 10, Shuffle::DealWithIncrement(3)));

    assert_eq!(0, reindex(0, 10, Shuffle::DealWithIncrement(7)));
    assert_eq!(7, reindex(1, 10, Shuffle::DealWithIncrement(7)));
    assert_eq!(1, reindex(3, 10, Shuffle::DealWithIncrement(7)));

    // 0 1 2 3 4 5 6 7 8 9
    // 0 9 8 7 6 5 4 3 2 1
    assert_eq!(9, reindex(1, 10, Shuffle::DealWithIncrement(9)));

    let seq = parse(EX1);
    assert_eq!(1, reindex_seq(3, 10, &seq));

    let seq = parse(EX2);
    assert_eq!(0, reindex_seq(3, 10, &seq));
}

fn revindex(i: i32, size: i32, shuffle: Shuffle) -> i32 {
    use Shuffle::*;
    match shuffle {
        DealIntoNewStack => (size - 1 - i) % size,
        Cut(n) => {
            let split = if n >= 0 { n } else { size + n };
            (i + split) % size
        }
        DealWithIncrement(n) => {
            if n == size - 1 {
                if i == 0 {
                    0
                } else {
                    (n * i) % size
                }
            } else {
                ((size - n) * i) % size
            }
        }
    }
}

#[test]
fn test_revindexing() {
    assert_eq!(3, revindex(6, 10, Shuffle::DealIntoNewStack));

    // 0 1 2 3 4 5 6 7 8 9
    // 3 4 5 6 7 8 9 0 1 2
    assert_eq!(1, revindex(8, 10, Shuffle::Cut(3)));

    // 0 1 2 3 4 5 6 7 8 9
    // 6 7 8 9 0 1 2 3 4 5
    assert_eq!(1, revindex(5, 10, Shuffle::Cut(-4)));
    assert_eq!(8, revindex(2, 10, Shuffle::Cut(-4)));

    // 0 1  2  3  4   5 6  7  8  9
    // 0 7 14 21 28 35 42 49 56 63
    assert_eq!(3, reindex(1, 10, Shuffle::DealWithIncrement(3)));
    assert_eq!(1, revindex(3, 10, Shuffle::DealWithIncrement(3)));

    assert_eq!(0, revindex(0, 10, Shuffle::DealWithIncrement(7)));
    assert_eq!(1, revindex(7, 10, Shuffle::DealWithIncrement(7)));
    assert_eq!(3, revindex(1, 10, Shuffle::DealWithIncrement(7)));

    // 0 1 2 3 4 5 6 7 8 9
    // 0 9 8 7 6 5 4 3 2 1
    assert_eq!(9, reindex(1, 10, Shuffle::DealWithIncrement(9)));
    assert_eq!(1, revindex(9, 10, Shuffle::DealWithIncrement(9)));

    let seq = parse(EX1);
    assert_eq!(3, revindex_seq(1, 10, &seq));

    let seq = parse(EX2);
    assert_eq!(3, revindex_seq(0, 10, &seq));
}

#[test]
fn test_revindex_seq() {
    let seq = parse(EX1);
    reindex_seq(3, 10, &seq);
    revindex_seq(1, 10, &seq);
}

fn reindex_seq(mut i: i32, size: i32, seq: &[Shuffle]) -> i32 {
    for &shuffle in seq {
        i = reindex(i, size, shuffle);
    }
    i
}

fn revindex_seq(mut i: i32, size: i32, seq: &[Shuffle]) -> i32 {
    for &shuffle in seq.iter().rev() {
        i = revindex(i, size, shuffle);
    }
    i
}
