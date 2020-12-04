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
            if p1_count == 7 {
                p1 += 1;
            }

            if p2_count == 7 {
                p2 += 1;
            }

            p1_count = 0;
            p2_count = 0;
        } else {
            let mut keys = line.split(' ').map(|kvp| (&kvp[..3], kvp[4..].trim_end()));

            'outer: while let Some((key, value)) = keys.next() {
                if key != "cid" {
                    p1_count += 1;
                }

                match key {
                    "byr" => {
                        let year: u16 = parse!(value);

                        if year >= 1920 && year <= 2002 {
                            p2_count += 1;
                        }
                    }
                    "iyr" => {
                        let year: u16 = parse!(value);

                        if year >= 2010 && year <= 2020 {
                            p2_count += 1;
                        }
                    }
                    "eyr" => {
                        let year: u16 = parse!(value);

                        if year >= 2020 && year <= 2030 {
                            p2_count += 1;
                        }
                    }
                    "hgt" => {
                        let (num, unit) = value.split_at(value.len() - 2);
                        let num: u8 = parse!(num);

                        let valid = match unit {
                            "cm" => num >= 150 && num <= 193,
                            "in" => num >= 59 && num <= 76,
                            _ => continue,
                        };

                        if valid {
                            p2_count += 1;
                        }
                    }
                    "hcl" => {
                        let value = value.as_bytes();

                        if value.len() != 7 || value[0] != b'#' {
                            continue;
                        }

                        for i in 1..7 {
                            if value[i] < b'0'
                                || value[i] > b'f'
                                || (value[i] > b'9' && value[i] < b'a')
                            {
                                continue 'outer;
                            }
                        }

                        p2_count += 1;
                    }
                    "ecl" => {
                        if ecl.contains(value) {
                            p2_count += 1;
                        }
                    }
                    "pid" => {
                        if value.len() != 9 || value.parse::<u32>().is_err() {
                            continue;
                        }

                        p2_count += 1;
                    }
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
