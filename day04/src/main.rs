use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::time::Instant;

macro_rules! parse {
    ($n:ident) => {
        match $n.parse() {
            Ok(n) => n,
            Err(_) => continue,
        }
    };
}

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1_count = 0;
    let mut p1 = 0;

    let mut p2_count = 0;
    let mut p2 = 0;

    let mut ecl = HashSet::with_capacity(7);
    ecl.insert("amb");
    ecl.insert("blu");
    ecl.insert("brn");
    ecl.insert("gry");
    ecl.insert("grn");
    ecl.insert("hzl");
    ecl.insert("oth");

    while input.read_line(&mut line).unwrap() != 0 {
        if line.trim_end().is_empty() {
            p1 += (p1_count == 7) as u16;
            p2 += (p2_count == 7) as u16;

            p1_count = 0;
            p2_count = 0;
        } else {
            let mut keys = line
                .split(' ')
                .map(|kvp| unsafe { (kvp.get_unchecked(..3), kvp.get_unchecked(4..).trim_end()) });

            'outer: while let Some((key, value)) = keys.next() {
                p1_count += (key != "cid") as u16;

                match key {
                    "byr" => {
                        let year: u16 = parse!(value);
                        p2_count += (year >= 1920 && year <= 2002) as u16;
                    }
                    "iyr" => {
                        let year: u16 = parse!(value);
                        p2_count += (year >= 2010 && year <= 2020) as u16;
                    }
                    "eyr" => {
                        let year: u16 = parse!(value);
                        p2_count += (year >= 2020 && year <= 2030) as u16;
                    }
                    "hgt" => {
                        let (num, unit) = value.split_at(value.len() - 2);
                        let num: u8 = parse!(num);

                        let valid = match unit {
                            "cm" => num >= 150 && num <= 193,
                            "in" => num >= 59 && num <= 76,
                            _ => continue,
                        };

                        p2_count += valid as u16;
                    }
                    "hcl" => {
                        let value = value.as_bytes();

                        if value.len() != 7 || unsafe { *value.get_unchecked(0) } != b'#' {
                            continue;
                        }

                        for i in 1..7 {
                            let byte = unsafe { *value.get_unchecked(i) };
                            if byte < b'0' || byte > b'f' || (byte > b'9' && byte < b'a') {
                                continue 'outer;
                            }
                        }

                        p2_count += 1;
                    }
                    "ecl" => p2_count += ecl.contains(value) as u16,
                    "pid" => p2_count += (value.len() == 9 && value.parse::<u32>().is_ok()) as u16,
                    _ => {}
                }
            }
        }

        line.clear();
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 1.2ms

    assert_eq!(p1, 216);
    assert_eq!(p2, 150);
}
