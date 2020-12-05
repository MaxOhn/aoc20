use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut seats = [false; 1024];

    while input.read_line(&mut line).unwrap() != 0 {
        let bytes = line.as_bytes();

        let mut row = 0;
        let mut pow = 64;

        let mut i = 0;
        while i != 7 {
            row += (bytes[i] == b'B') as usize * pow;
            pow /= 2;
            i += 1;
        }

        let mut col = 0;
        let mut pow = 4;

        while i != 10 {
            col += (bytes[i] == b'R') as usize * pow;
            pow /= 2;
            i += 1;
        }

        let id = 8 * row + col;

        p1 = p1.max(id);
        seats[id] = true;

        line.clear();
    }

    let mut p2 = 1;
    loop {
        if !seats[p2] {
            if seats[p2 - 1] && seats[p2 + 1] {
                break;
            }
            p2 += 2;
        } else {
            p2 += 1;
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 730µs

    assert_eq!(p1, 922);
    assert_eq!(p2, 747);
}
