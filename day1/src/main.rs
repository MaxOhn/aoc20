use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut numbers = Vec::with_capacity(128);

    while matches!(input.read_line(&mut line), Ok(n) if n != 0) {
        numbers.push(line.trim().parse().unwrap());
        line.clear();
    }

    part1(&numbers); // 2.1µs
    part2(&numbers); // 23.5µs
}

fn part1(numbers: &[u32]) {
    let start = std::time::Instant::now();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return println!(
                    "Part 1: {} [{:?}]",
                    numbers[i] * numbers[j],
                    start.elapsed()
                );
            }
        }
    }
}

fn part2(numbers: &[u32]) {
    let start = std::time::Instant::now();
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] > 2020 {
                continue;
            }
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return println!(
                        "Part 2: {} [{:?}]",
                        numbers[i] * numbers[j] * numbers[k],
                        start.elapsed()
                    );
                }
            }
        }
    }
}
