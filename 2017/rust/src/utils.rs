use std::io::{self, Read};

fn read_stdin() -> String {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    s
}

pub fn read_number() -> Result<i32, std::num::ParseIntError> {
    read_stdin().trim().parse()
}
