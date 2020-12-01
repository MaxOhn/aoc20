use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::with_capacity(5);
    let mut numbers = Vec::with_capacity(200);

    while input.read_line(&mut line).unwrap() != 0 {
        numbers.push(line.trim().parse().unwrap());
        line.clear();
    }

    numbers.sort();

    println!("Setup: {:?}", start.elapsed()); // 103µs

    part1(&numbers); // 600ns
    part2(&numbers); // 2.5µs
}

fn part1(numbers: &[u32]) {
    let start = Instant::now();
    for i in 0..numbers.len() {
        if let Ok(j) = numbers[i + 1..].binary_search(&(2020 - numbers[i])) {
            return println!(
                "Part 1: {} [{:?}]",
                numbers[i] * numbers[j + i + 1],
                start.elapsed()
            );
        }
    }
}

fn part2(numbers: &[u32]) {
    let start = Instant::now();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] > 2020 {
                break;
            }
            if let Ok(k) = numbers[j + 1..].binary_search(&(2020 - numbers[i] - numbers[j])) {
                return println!(
                    "Part 2: {} [{:?}]",
                    numbers[i] * numbers[j] * numbers[k + j + 1],
                    start.elapsed()
                );
            }
        }
    }
}
