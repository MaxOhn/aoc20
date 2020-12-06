use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    static mut QUESTIONS: [u8; 26] = [0; 26];

    let mut p1 = 0;
    let mut p2 = 0;
    let mut group_size = 0;

    while input.read_line(&mut line).unwrap() != 0 {
        let bytes = line.trim_end().as_bytes();
        let mut i = 0;

        if bytes.is_empty() {
            while i != 26 {
                unsafe {
                    p1 += (*QUESTIONS.get_unchecked(i) > 0) as u16;
                    p2 += (*QUESTIONS.get_unchecked(i) == group_size) as u16;
                    *QUESTIONS.get_unchecked_mut(i) = 0;
                }
                i += 1;
            }
            group_size = 0;
        } else {
            while i != bytes.len() {
                unsafe {
                    *QUESTIONS.get_unchecked_mut((*bytes.get_unchecked(i) - b'a') as usize) += 1
                }
                i += 1;
            }
            group_size += 1;
            line.clear();
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 611µs

    assert_eq!(p1, 6742);
    assert_eq!(p2, 3447);
}
