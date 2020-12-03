use std::io::{BufRead, BufReader};
use std::time::Instant;

#[rustfmt::skip]
static mut COUNTERS: [Counter; 4] = [
    Counter { step: 1, x: 0, count: 0 },
    Counter { step: 3, x: 0, count: 0 },
    Counter { step: 5, x: 0, count: 0 },
    Counter { step: 7, x: 0, count: 0 },
];

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut skipper = 0;
    let mut skipper_x = 0;
    let mut y = 0;

    while input.read_line(&mut line).unwrap() != 0 {
        let trimmed = line.trim_end();

        for counter in unsafe { COUNTERS.iter_mut() } {
            counter.update(trimmed);
        }

        if y % 2 == 0 {
            if trimmed.as_bytes()[skipper_x] == b'#' {
                skipper += 1;
            }
            skipper_x = (skipper_x + 1) % trimmed.len();
        }
        y += 1;

        line.clear();
    }

    let p1 = unsafe { COUNTERS[1].count };
    let p2 = unsafe { COUNTERS.iter() }
        .map(|counter| counter.count)
        .fold(skipper, |product, count| product * count);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 650µs

    assert_eq!(p1, 156);
    assert_eq!(p2, 3_521_829_480);
}

struct Counter {
    step: usize,
    x: usize,
    count: u32,
}

impl Counter {
    fn update(&mut self, line: &str) {
        if line.as_bytes()[self.x as usize] == b'#' {
            self.count += 1;
        }
        self.x = (self.x + self.step) % line.len();
    }
}
