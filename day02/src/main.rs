use std::io::{BufRead, BufReader};
use std::time::Instant;

#[cfg(not(feature = "reg"))]
fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut p2 = 0;

    while input.read_line(&mut line).unwrap() != 0 {
        let mut split = line.split('-');
        let min = split.next().unwrap().parse().unwrap();
        let mut split = split.next().unwrap().split(' ');
        let max = split.next().unwrap().parse().unwrap();
        let letter = split.next().unwrap().as_bytes()[0];
        let password = split.next().unwrap().as_bytes();

        if part1(min, max, letter, password) {
            p1 += 1;
        }

        if part2(min - 1, max - 1, letter, password) {
            p2 += 1;
        }

        line.clear();
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 1.5ms

    assert_eq!(p1, 582);
    assert_eq!(p2, 729);
}

fn part1(min: usize, max: usize, letter: u8, password: &[u8]) -> bool {
    let count = password.iter().filter(|c| **c == letter).count();

    count >= min && count <= max
}

fn part2(min: usize, max: usize, letter: u8, password: &[u8]) -> bool {
    match (password.get(min), password.get(max)) {
        (Some(a), Some(b)) => (*a == letter) ^ (*b == letter),
        (Some(a), None) => *a == letter,
        (None, _) => false,
    }
}

#[cfg(feature = "rgx")]
fn main() {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref LINE_MATCHER: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)\n?").unwrap();
    }

    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut p2 = 0;

    while input.read_line(&mut line).unwrap() != 0 {
        let caps = LINE_MATCHER.captures(&line).unwrap();

        let min = caps[1].parse().unwrap();
        let max = caps[2].parse().unwrap();
        let letter = caps[3].as_bytes()[0];
        let password = caps[4].as_bytes();

        if part1(min, max, letter, password) {
            p1 += 1;
        }

        if part2(min - 1, max - 1, letter, password) {
            p2 += 1;
        }

        line.clear();
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 6ms

    assert_eq!(p1, 582);
    assert_eq!(p2, 729);
}
