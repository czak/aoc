#![allow(non_upper_case_globals)]
#![allow(unused_variables, dead_code)]

type Op = fn(usize, usize, usize, &mut [u64; 6]);

const addr: Op = |a, b, c, r| r[c] = r[a] + r[b];
const addi: Op = |a, b, c, r| r[c] = r[a] + b as u64;
const mulr: Op = |a, b, c, r| r[c] = r[a] * r[b];
const muli: Op = |a, b, c, r| r[c] = r[a] * b as u64;
const banr: Op = |a, b, c, r| r[c] = r[a] & r[b];
const bani: Op = |a, b, c, r| r[c] = r[a] & b as u64;
const borr: Op = |a, b, c, r| r[c] = r[a] | r[b];
const bori: Op = |a, b, c, r| r[c] = r[a] | b as u64;
const setr: Op = |a, b, c, r| r[c] = r[a];
const seti: Op = |a, b, c, r| r[c] = a as u64;
const gtir: Op = |a, b, c, r| r[c] = if a as u64 > r[b] { 1 } else { 0 };
const gtri: Op = |a, b, c, r| r[c] = if r[a] > b as u64 { 1 } else { 0 };
const gtrr: Op = |a, b, c, r| r[c] = if r[a] > r[b] { 1 } else { 0 };
const eqir: Op = |a, b, c, r| r[c] = if a as u64 == r[b] { 1 } else { 0 };
const eqri: Op = |a, b, c, r| r[c] = if r[a] == b as u64 { 1 } else { 0 };
const eqrr: Op = |a, b, c, r| r[c] = if r[a] == r[b] { 1 } else { 0 };

const IP_BINDING: usize = 0;

fn main() {
    let program = [
        (seti, 5, 0, 1),
        (seti, 6, 0, 2),
        (addi, 0, 1, 0),
        (addr, 1, 2, 3),
        (setr, 1, 0, 0),
        (seti, 8, 0, 4),
        (seti, 9, 0, 5),
    ];

    let program = [
        (addi, 2, 16, 2),
        (seti, 1, 0, 1),
        (seti, 1, 4, 3),
        (mulr, 1, 3, 4),
        (eqrr, 4, 5, 4),
        (addr, 4, 2, 2),
        (addi, 2, 1, 2),
        (addr, 1, 0, 0),
        (addi, 3, 1, 3),
        (gtrr, 3, 5, 4),
        (addr, 2, 4, 2),
        (seti, 2, 5, 2),
        (addi, 1, 1, 1),
        (gtrr, 1, 5, 4),
        (addr, 4, 2, 2),
        (seti, 1, 1, 2),
        (mulr, 2, 2, 2),
        (addi, 5, 2, 5),
        (mulr, 5, 5, 5),
        (mulr, 2, 5, 5),
        (muli, 5, 11, 5),
        (addi, 4, 5, 4),
        (mulr, 4, 2, 4),
        (addi, 4, 9, 4),
        (addr, 5, 4, 5),
        (addr, 2, 0, 2),
        (seti, 0, 0, 2),
        (setr, 2, 3, 4),
        (mulr, 4, 2, 4),
        (addr, 2, 4, 4),
        (mulr, 2, 4, 4),
        (muli, 4, 14, 4),
        (mulr, 4, 2, 4),
        (addr, 5, 4, 5),
        (seti, 0, 6, 0),
        (seti, 0, 3, 2),
    ];

    let mut r: [u64; 6] = [0, 0, 0, 0, 0, 0];
    let mut ip: usize = 0;

    println!("ip={}, r={:?}", ip, r);

    while ip < program.len() {
        r[IP_BINDING] = ip as u64;
        let instr = program[ip];
        let op = instr.0;
        op(instr.1, instr.2, instr.3, &mut r);
        ip = r[IP_BINDING] as usize;
        ip += 1;

        println!("ip={}, r={:?}", ip, r);
    }
}
