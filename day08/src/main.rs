use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut instructions: Vec<Op> = Vec::with_capacity(640);

    while input.read_line(&mut line).unwrap() != 0 {
        instructions.push(line.parse().unwrap());
        line.clear();
    }

    let mut seen = HashSet::with_capacity(128);

    println!("Setup: {:?}", start.elapsed()); // 176µs

    let start = Instant::now();
    let p1 = part1(&instructions, &mut seen).unwrap_err();
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 307µs

    let start = Instant::now();
    let p2 = part2(&mut instructions, &mut seen);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 1.1ms

    assert_eq!(p1, 2014);
    assert_eq!(p2, 2251);
}

fn part1(instructions: &[Op], seen: &mut HashSet<i32>) -> Result<i32, i32> {
    let mut acc = 0;
    let mut pc = 0;
    let mut prev;

    while (pc as usize) < instructions.len() {
        prev = acc;

        match unsafe { instructions.get_unchecked(pc as usize) } {
            Op::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Op::Jmp(n) => pc += n,
            Op::Nop(_) => pc += 1,
        }

        if !seen.insert(pc) {
            return Err(prev);
        }
    }

    Ok(acc)
}

fn part2(instructions: &mut [Op], seen: &mut HashSet<i32>) -> i32 {
    let mut i = instructions.len() - 1;

    loop {
        let replaced = loop {
            match unsafe { *instructions.get_unchecked(i) } {
                Op::Acc(_) => i -= 1,
                Op::Jmp(n) => {
                    unsafe { *instructions.get_unchecked_mut(i) = Op::Nop(n) }
                    break Op::Jmp(n);
                }
                Op::Nop(n) => {
                    unsafe { *instructions.get_unchecked_mut(i) = Op::Jmp(n) }
                    break Op::Nop(n);
                }
            }
        };

        seen.clear();

        match part1(&instructions, seen) {
            Ok(n) => return n,
            Err(_) => unsafe { *instructions.get_unchecked_mut(i) = replaced },
        }

        i -= 1;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = unsafe { s.trim_end().get_unchecked(4..) }.parse().unwrap();

        let op = match unsafe { s.as_bytes().get_unchecked(0) } {
            b'a' => Self::Acc(n),
            b'j' => Self::Jmp(n),
            b'n' => Self::Nop(n),
            _ => unreachable!(),
        };

        Ok(op)
    }
}
