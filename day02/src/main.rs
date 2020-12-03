use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut p2 = 0;

    let delims: [char; 3] = ['-', ':', ' '];

    while input.read_line(&mut line).unwrap() != 0 {
        let mut split = line.split(delims.as_ref());

        let min = split.next().unwrap().parse().unwrap();
        let max = split.next().unwrap().parse().unwrap();
        let letter = split.next().unwrap().chars().next().unwrap();
        let password = split.nth(1).unwrap().trim_end();

        if part1(min, max, letter, password) {
            p1 += 1;
        }

        if part2(min, max, letter, password) {
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

fn part1(min: usize, max: usize, letter: char, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == letter).count();

    count >= min && count <= max
}

fn part2(min: usize, max: usize, letter: char, password: &str) -> bool {
    let mut chars = password.chars();

    match (chars.nth(min - 1), chars.nth(max - min - 1)) {
        (Some(a), Some(b)) => (a == letter) ^ (b == letter),
        (Some(a), None) => a == letter,
        (None, _) => false,
    }
}
