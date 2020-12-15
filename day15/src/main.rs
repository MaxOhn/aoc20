use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let p1 = run(2020);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 109µs

    let start = Instant::now();
    let p2 = run(30_000_000);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 546µs

    assert_eq!(p1, 1665);
    assert_eq!(p2, 16_439);
}

fn run(size: usize) -> u32 {
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();

    let mut nums = vec![u32::MAX; size];

    let mut i = 0;
    let mut idx = 0;
    let mut last = 0;
    let end = size as u32 - 1;

    while i < bytes.len() {
        let byte = unsafe { *bytes.get_unchecked(i) };
        if byte == b',' {
            unsafe { *nums.get_unchecked_mut(last as usize) = idx }
            last = 0;
            idx += 1;
        } else if byte == b'\n' {
            break;
        } else {
            last = last * 10 + (byte & 0x0F) as u32;
        }

        i += 1;
    }

    while idx < end {
        let mut prev = idx;
        std::mem::swap(&mut prev, unsafe { nums.get_unchecked_mut(last as usize) });
        last = idx.saturating_sub(prev);
        idx += 1;
    }

    last
}

#[allow(dead_code)]
fn run_old(size: usize) -> usize {
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();

    let mut nums = vec![None; size];

    let mut i = 0;
    let mut idx = 0;
    let mut last = 0;
    let end = size - 1;

    while i < bytes.len() {
        let byte = unsafe { *bytes.get_unchecked(i) };
        if byte == b',' {
            unsafe { *nums.get_unchecked_mut(last) = Some(idx) }
            last = 0;
            idx += 1;
        } else if byte == b'\n' {
            break;
        } else {
            last = last * 10 + (byte & 0x0F) as usize;
        }

        i += 1;
    }

    while idx < end {
        let last_idx = unsafe { *nums.get_unchecked(last) };
        unsafe { *nums.get_unchecked_mut(last) = Some(idx) }
        last = last_idx.map_or(0, |n| idx - n);
        idx += 1;
    }

    last
}
