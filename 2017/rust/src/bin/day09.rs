use std::io::{self, Read};

fn score_and_clean(s: &str) -> (i32, i32) {
    let mut iter = s.chars();

    let mut score = 0;
    let mut garbage = 0;

    let mut level = 0;
    let mut in_garbage = false;

    while let Some(c) = iter.next() {
        if in_garbage {
            match c {
                '>' => { in_garbage = false; },
                '!' => { iter.next(); },
                _ => { garbage += 1; },
            }
        } else {
            match c {
                '{' => { level += 1; },
                '}' => { score += level; level -= 1; },
                '<' => { in_garbage = true; },
                _ => continue,
            }
        }
    }

    (score, garbage)
}

fn main() {
    assert_eq!((1, 0), score_and_clean("{}")); 
    assert_eq!((6, 0), score_and_clean("{{{}}}")); 
    assert_eq!((5, 0), score_and_clean("{{},{}}")); 
    assert_eq!((16, 0), score_and_clean("{{{},{},{{}}}}"));
    assert_eq!((1, 4), score_and_clean("{<a>,<a>,<a>,<a>}"));
    assert_eq!((9, 8), score_and_clean("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!((9, 0), score_and_clean("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!((3, 17), score_and_clean("{{<a!>},{<a!>},{<a!>},{<ab>}}"));

    assert_eq!((1, 0), score_and_clean("{}"));
    assert_eq!((1, 17), score_and_clean("{<random characters>}"));
    assert_eq!((1, 3), score_and_clean("{<<<<>}"));
    assert_eq!((1, 2), score_and_clean("{<{!>}>}"));
    assert_eq!((1, 0), score_and_clean("{<!!>}"));
    assert_eq!((1, 0), score_and_clean("{<!!!>>}"));
    assert_eq!((1, 10), score_and_clean(r#"{<{o"i!a,<{i<a>}"#));

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let (score, garbage) = score_and_clean(&s);
    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage);
}
