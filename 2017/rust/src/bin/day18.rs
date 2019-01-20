use std::collections::HashMap;
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Op {
    sndi(i64),
    sndr(char),
    seti(char, i64),
    setr(char, char),
    addi(char, i64),
    addr(char, char),
    muli(char, i64),
    mulr(char, char),
    modi(char, i64),
    modr(char, char),
    rcvi(i64),
    rcvr(char),
    jgzii(i64, i64),
    jgzir(i64, char),
    jgzri(char, i64),
    jgzrr(char, char),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Op::*;

        let opcode = &s[0..3];
        let args: Vec<&str> = s[4..].split(' ').collect();

        match opcode {
            "snd" => {
                if let Ok(num) = args[0].parse::<i64>() {
                    Ok(sndi(num))
                } else {
                    let x = args[0].chars().nth(0).unwrap();
                    Ok(sndr(x))
                }
            },
            "set" => {
                let x: char = args[0].chars().nth(0).unwrap();
                if let Ok(num) = args[1].parse::<i64>() {
                    Ok(seti(x, num))
                } else {
                    let y = args[1].chars().nth(0).unwrap();
                    Ok(setr(x, y))
                }
            },
            "add" => {
                let x: char = args[0].chars().nth(0).unwrap();
                if let Ok(num) = args[1].parse::<i64>() {
                    Ok(addi(x, num))
                } else {
                    let y = args[1].chars().nth(0).unwrap();
                    Ok(addr(x, y))
                }
            },
            "mul" => {
                let x: char = args[0].chars().nth(0).unwrap();
                if let Ok(num) = args[1].parse::<i64>() {
                    Ok(muli(x, num))
                } else {
                    let y = args[1].chars().nth(0).unwrap();
                    Ok(mulr(x, y))
                }
            },
            "mod" => {
                let x: char = args[0].chars().nth(0).unwrap();
                if let Ok(num) = args[1].parse::<i64>() {
                    Ok(modi(x, num))
                } else {
                    let y = args[1].chars().nth(0).unwrap();
                    Ok(modr(x, y))
                }
            },
            "rcv" => {
                if let Ok(num) = args[0].parse::<i64>() {
                    Ok(rcvi(num))
                } else {
                    let x = args[0].chars().nth(0).unwrap();
                    Ok(rcvr(x))
                }
            },
            "jgz" => {
                if let Ok(num1) = args[0].parse::<i64>() {
                    if let Ok(num2) = args[1].parse::<i64>() {
                        Ok(jgzii(num1, num2))
                    } else {
                        let y = args[1].chars().nth(0).unwrap();
                        Ok(jgzir(num1, y))
                    }
                } else {
                    let x = args[0].chars().nth(0).unwrap();
                    if let Ok(num2) = args[1].parse::<i64>() {
                        Ok(jgzri(x, num2))
                    } else {
                        let y = args[1].chars().nth(0).unwrap();
                        Ok(jgzrr(x, y))
                    }
                }
            }
            _ => Err(())
        }
    }
}

// const input: &str = "set a 1
// add a 2
// mul a a
// mod a 5
// snd a
// set a 0
// rcv a
// jgz a -1
// set a 1
// jgz a -2";

#[allow(non_upper_case_globals)]
const input: &str = "set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 826
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19";

// const input: &str = "snd 1
// snd 2
// snd p
// rcv a
// rcv b
// rcv c
// rcv d";

fn run_program(num: i64, tx: mpsc::Sender<i64>, rx: mpsc::Receiver<i64>) {
    let program: Vec<Op> = input.lines().map(|s| Op::from_str(s).unwrap()).collect();
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut ip: i64 = 0;
    let mut send_count: i64 = 0;

    for c in ('a' as u8)..=('z' as u8) {
        registers.insert(c as char, 0);
    }

    // Init register p
    registers.insert('p', num);

    while ip >= 0 && ip < program.len() as i64 {
        match &program[ip as usize] {
            &Op::sndi(x) => {
                tx.send(x).unwrap();

                send_count += 1;
                println!("p{} sent {} (#{})", num, x, send_count);
            },
            &Op::sndr(x) => {
                tx.send(registers[&x]).unwrap();

                send_count += 1;
                println!("p{} sent {} (#{})", num, registers[&x], send_count);
            },
            &Op::seti(x, y) => {
                registers.insert(x, y);
            },
            &Op::setr(x, y) => {
                registers.insert(x, registers[&y]);
            },
            &Op::addi(x, y) => {
                let val = registers[&x];
                registers.insert(x, val + y);
            },
            &Op::addr(x, y) => {
                let val = registers[&x];
                registers.insert(x, val + registers[&y]);
            },
            &Op::muli(x, y) => {
                let val = registers[&x];
                registers.insert(x, val * y);
            },
            &Op::mulr(x, y) => {
                let val = registers[&x];
                registers.insert(x, val * registers[&y]);
            },
            &Op::modi(x, y) => {
                let val = registers[&x];
                registers.insert(x, val % y);
            },
            &Op::modr(x, y) => {
                let val = registers[&x];
                registers.insert(x, val % registers[&y]);
            },
            &Op::rcvi(_) => {
                panic!("rcvi not allowed");
            },
            &Op::rcvr(x) => {
                println!("p{} receiving...", num);

                match rx.recv_timeout(std::time::Duration::from_millis(500)) {
                    Ok(val) => {
                        registers.insert(x, val);
                        println!("p{} received {}", num, val);
                    },
                    Err(_) => {
                        println!("p{} timed out.", num);
                        break;
                    },
                }
            },
            &Op::jgzii(x, y) => {
                if x > 0 {
                    ip += y;
                    continue;
                }
            },
            &Op::jgzir(x, y) => {
                if x > 0 {
                    ip += registers[&y];
                    continue;
                }
            },
            &Op::jgzri(x, y) => {
                if registers[&x] > 0 {
                    ip += y;
                    continue;
                }
            },
            &Op::jgzrr(x, y) => {
                if registers[&x] > 0 {
                    ip += registers[&y];
                    continue;
                }
            },
        }
        ip += 1;
    }

    println!("p{} sent a total of {} times.", num, send_count);
}

fn main() {
    let (tx0, rx0) = mpsc::channel::<i64>();
    let (tx1, rx1) = mpsc::channel::<i64>();

    let thread0 = thread::spawn(move || {
        run_program(0, tx0, rx1);
    });

    let thread1 = thread::spawn(move || {
        run_program(1, tx1, rx0);
    });

    thread0.join().unwrap();
    thread1.join().unwrap();
}
