fn valid(mut num: u32) -> bool {
    let mut adjacent = false;

    let mut prev = 10;

    for _ in 0..6 {
        let current = num % 10;

        if current > prev {
            return false;
        }
        if current == prev {
            adjacent = true;
        }

        num /= 10;
        prev = current;
    }

    adjacent
}

fn valid2(mut num: u32) -> bool {
    let mut adjacent = 0;
    let mut valid_adjacent = false;

    let mut prev = 10;

    for _ in 0..6 {
        let current = num % 10;

        if current > prev {
            return false;
        }

        if current == prev {
            adjacent += 1;
        } else {
            if adjacent == 1 {
                valid_adjacent = true;
            }
            adjacent = 0;
        }

        num /= 10;
        prev = current;
    }

    valid_adjacent || adjacent == 1
}

fn main() {
    let range = 193651..=649729;

    assert!(valid(111111));
    assert!(!valid(223450));
    assert!(!valid(123789));

    let count = range.clone().filter(|&n| valid(n)).count();

    println!("Part 1: {}", count);
    assert_eq!(1605, count);

    assert!(valid2(112233));
    assert!(!valid2(123444));
    assert!(valid2(111122));
    assert!(valid2(112222));

    let count = range.clone().filter(|&n| valid2(n)).count();

    println!("Part 2: {}", count);
    assert_eq!(1102, count);
}
