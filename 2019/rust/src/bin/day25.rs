use aoc2019::intcode_redux as intcode;

fn main() {
    let mut ic = intcode::Intcode {
        mem: intcode::parse(include_str!("../../../in/day25.in")),
        input: "inv\n".chars().map(|c| c as i64).collect(),
        ..Default::default()
    };
    ic.mem.resize(10000, 0);

    while let Some(n) = ic.next() {
        print!("{}", char::from(n as u8));
    }

    // dbg!(ic.collect::<Vec<_>>());
}

// east
// south
// west
// north
// o
//
// north from holodeck
//
//
//      5    ?
//      |    |
//    4-3-0---1
//      |     |
//      2- ?
//      |
//      6
//
// 0 - hull breach (esw)
// 1 - passages (nsw)
// 2 - hallway (nes) (escape pod !!!)
// 3 - gift wrapping center (new) (mug)
// 4 - navigation (e)
// 5 - warp drive maintenance (s) (easter egg)
// 6 - stables (n) (infinite loop !!!)
//
//
// == Pressure-Sensitive Floor ==
// Analyzing...
//
// Doors here lead:
// - west
//
// A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
// Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.
// "Oh, hello! You should be able to get in by typing 1077936448 on the keypad at the main airlock."
