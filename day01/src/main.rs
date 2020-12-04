use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::with_capacity(5);
    let mut numbers = Vec::with_capacity(200);

    while input.read_line(&mut line).unwrap() != 0 {
        numbers.push(line.trim_end().parse().unwrap());
        line.clear();
    }

    numbers.sort_unstable();

    println!("Setup: {:?}", start.elapsed()); // 103µs

    let start = Instant::now();
    let p1 = part1(&numbers);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 600ns

    let start = Instant::now();
    let p2 = part2(&numbers);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 2.5µs

    assert_eq!(p1, 326211);
    assert_eq!(p2, 131347190);
}

fn part1(numbers: &[u32]) -> u32 {
    for i in 0..numbers.len() {
        if let Ok(j) = numbers[i + 1..].binary_search(&(2020 - numbers[i])) {
            return numbers[i] * numbers[j + i + 1];
        }
    }
    unreachable!()
}

fn part2(numbers: &[u32]) -> u32 {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] > 2020 {
                break;
            }
            if let Ok(k) = numbers[j + 1..].binary_search(&(2020 - numbers[i] - numbers[j])) {
                return numbers[i] * numbers[j] * numbers[k + j + 1];
            }
        }
    }
    unreachable!()
}
