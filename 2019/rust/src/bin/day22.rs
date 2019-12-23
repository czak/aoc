#![allow(dead_code)]
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Shuffle {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

impl FromStr for Shuffle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            return Ok(Shuffle::DealIntoNewStack);
        }
        let n: i128 = s.rsplit(' ').next().unwrap().parse().unwrap();
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

    let mut dseen: HashSet<i128> = HashSet::new();
    let mut nseen: HashSet<i128> = HashSet::new();

    // Part 2
    let mut n = 2020;
    let mut prev;
    for i in 0..100000 {
        prev = n;
        n = revindex_seq(n, 119315717514047, &seq);

        // change from prev
        let diff = if n >= prev {
            n - prev
        } else {
            n - prev + 119315717514047
        };

        println!("{}, {}", n, diff);

        if nseen.contains(&n) {
            println!("{}: nseen {}", i, n);
        } else {
            nseen.insert(n);
        }

        if dseen.contains(&diff) {
            println!("{}: dseen {}", i, diff);
        } else {
            dseen.insert(diff);
        }
    }
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

fn reindex(i: i128, size: i128, shuffle: Shuffle) -> i128 {
    use Shuffle::*;
    let res = match shuffle {
        DealIntoNewStack => (size - 1 - i) % size,
        Cut(n) => {
            let split = if n >= 0 { n } else { size + n };
            (i + size - split) % size
        }
        DealWithIncrement(n) => (n * i) % size,
    };
    println!("{:?}: {}", shuffle, res);
    res
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

fn revindex(i: i128, size: i128, shuffle: Shuffle) -> i128 {
    use Shuffle::*;
    let res = match shuffle {
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
    };
    println!("Rev. {:?}: {}", shuffle, res);
    res
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

    assert_eq!(2519, reindex(9799, 10007, Shuffle::DealWithIncrement(36)));
    assert_eq!(9799, revindex(2519, 10007, Shuffle::DealWithIncrement(36)));

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
fn test_reverse() {
    let seq = parse(include_str!("../../../in/day22.in"));

    // run seq 3 times
    let n = reindex_seq(2019, 10007, &seq); // = 2519
    assert_eq!(2519, n);
    // let n = reindex_seq(n, 10007, &seq); // = 6343
    // assert_eq!(6343, n);
    // let n = reindex_seq(n, 10007, &seq); // = 4207
    // assert_eq!(4207, n);
    //
    println!("------------------------ REV ---------------------------");

    // reverse 3 times
    // let n = revindex_seq(n, 10007, &seq);
    // assert_eq!(6343, n);
    // let n = revindex_seq(n, 10007, &seq);
    // assert_eq!(2519, n);
    let n = revindex_seq(2519, 10007, &seq);
    assert_eq!(2019, n);
}

fn reindex_seq(mut i: i128, size: i128, seq: &[Shuffle]) -> i128 {
    for &shuffle in seq {
        i = reindex(i, size, shuffle);
    }
    i
}

fn revindex_seq(mut i: i128, size: i128, seq: &[Shuffle]) -> i128 {
    for &shuffle in seq.iter().rev() {
        i = revindex(i, size, shuffle);
    }
    i
}
