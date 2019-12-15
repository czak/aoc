use aoc2019::intcode_redux as intcode;
use maplit::hashset;
use std::collections::HashSet;

type Pos = (i32, i32);

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day15.in")),
        ..Default::default()
    };

    let mut walls: HashSet<Pos> = hashset!();
    let target = discover((0, 0), &mut ic, &mut hashset!(), &mut walls);
    dbg!(&walls);
    dbg!(&target);

    let xmin = walls.iter().min_by_key(|(x, _)| x).unwrap().0;
    let xmax = walls.iter().max_by_key(|(x, _)| x).unwrap().0;
    let ymin = walls.iter().min_by_key(|(_, y)| y).unwrap().1;
    let ymax = walls.iter().max_by_key(|(_, y)| y).unwrap().1;
    println!("{},{} - {},{}", xmin, ymin, xmax, ymax);
}

fn discover(
    pos: Pos,
    ic: &mut intcode::Intcode,
    seen: &mut HashSet<Pos>,
    walls: &mut HashSet<Pos>,
) -> (i32, i32) {
    println!("Discovering {:?}", pos);
    seen.insert(pos);

    // 1 2 3 4
    // N S W E
    for next in &[
        ((pos.0, pos.1 + 1), 1),
        ((pos.0, pos.1 - 1), 2),
        ((pos.0 - 1, pos.1), 3),
        ((pos.0 + 1, pos.1), 4),
    ] {
        // let next = ((pos.0, pos.1 + 1), 1);
        if !seen.contains(&next.0) {
            ic.input.push(next.1);
            let res = ic.next().unwrap();
            if res == 2 {
                return next.0;
            } else if res == 1 {
                return discover(next.0, ic, seen, walls);
            } else {
                walls.insert(next.0);
            }
        }
    }

    (-999, -999)
}
