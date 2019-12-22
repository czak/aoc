#![allow(dead_code)]
use std::str::FromStr;

#[derive(Debug)]
enum Shuffle {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(usize),
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
            Ok(Shuffle::DealWithIncrement(n as usize))
        }
    }
}

fn main() {
    // Part 1

    let deck: Vec<u32> = (0..).take(10007).collect();
    let seq = parse(include_str!("../../../in/day22.in"));
    let res = shuffle(deck, &seq);
    println!("Part 1: {:?}", res.iter().position(|&n| n == 2019).unwrap());

    // // Part 2
    //
    // let mut deck: Vec<u32> = (0..).take(10).collect();
    // let seq = parse(include_str!("../../../in/day22.in"));
    // deck = shuffle(deck, &seq);
}

fn shuffle(mut deck: Vec<u32>, seq: &Vec<Shuffle>) -> Vec<u32> {
    use Shuffle::*;
    for shuffle in seq {
        deck = match shuffle {
            &DealIntoNewStack => deck.into_iter().rev().collect(),
            &Cut(n) => {
                let split: usize = if n >= 0 {
                    n as usize
                } else {
                    (deck.len() as i32 + n) as usize
                };

                deck.iter()
                    .skip(split)
                    .chain(deck.iter().take(split))
                    .copied()
                    .collect()
            }
            &DealWithIncrement(n) => {
                let indexes = (0..).take(deck.len()).cycle().step_by(n).take(deck.len());
                let mut deck: Vec<u32> = deck.into_iter().rev().collect();
                let mut res = vec![];
                res.resize(deck.len(), 0);
                for i in indexes {
                    res[i] = deck.pop().unwrap();
                }
                res
            }
        };
        // println!("{:?}: {:?}", shuffle, deck);
    }
    deck
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

#[test]
fn test_shuffling() {
    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        shuffle(deck, &vec![Shuffle::DealIntoNewStack])
    );

    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(
        vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
        shuffle(deck, &vec![Shuffle::Cut(3)])
    );

    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(
        vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
        shuffle(deck, &vec![Shuffle::Cut(-4)])
    );

    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(
        vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        shuffle(deck, &vec![Shuffle::DealWithIncrement(3)])
    );

    let deck = vec![1, 0, 9, 8, 7, 6, 5, 4, 3, 2];
    assert_eq!(
        vec![1, 8, 5, 2, 9, 6, 3, 0, 7, 4],
        shuffle(deck, &vec![Shuffle::DealWithIncrement(7)])
    );

    let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        //   0  1  2  3  4  5  6  7  8  9
        vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        shuffle(deck, &vec![Shuffle::DealWithIncrement(9)])
    );

    let seq = parse(EX1);
    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], shuffle(deck, &seq));

    let seq = parse(EX2);
    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], shuffle(deck, &seq));

    let seq = parse(EX3);
    let deck: Vec<u32> = (0..).take(10).collect();
    assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], shuffle(deck, &seq));
}
