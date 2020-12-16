use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

macro_rules! byte {
    ($bytes:ident, $i:ident) => {
        unsafe { *$bytes.get_unchecked($i) }
    };
}

struct Rule {
    name: String,
    a: u16,
    b: u16,
    c: u16,
    d: u16,
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Rule {}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}-{} or {}-{}",
            self.name, self.a, self.b, self.c, self.d
        )
    }
}

impl Rule {
    fn new(name: &[u8], a: u16, b: u16, c: u16, d: u16) -> Self {
        let name = String::from_utf8_lossy(name).to_string();
        Self { name, a, b, c, d }
    }

    fn contains(&self, n: u16) -> bool {
        (n >= self.a && n <= self.b) || (n >= self.c && n <= self.d)
    }
}

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 25_788);
    assert_eq!(p2, 3_902_565_915_559);
}

fn part1() -> u16 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut rules = Vec::with_capacity(8);

    while {
        let _ = input.read_line(&mut line);
        line.len() > 1
    } {
        let bytes = line.as_bytes();
        let mut j = 0;

        while byte!(bytes, j) != b':' {
            j += 1;
        }

        let name = &[];

        let read_num = |j: &mut usize, end| {
            let mut n = 0;
            let mut byte;

            while *j < bytes.len() {
                byte = unsafe { *bytes.get_unchecked(*j) };

                if byte == end {
                    return n;
                }

                n = n * 10 + (byte & 0x0F) as u16;

                *j += 1;
            }

            n
        };

        j += 2;
        let a = read_num(&mut j, b'-');

        j += 1;
        let b = read_num(&mut j, b' ');

        j += 4;
        let c = read_num(&mut j, b'-');

        j += 1;
        let d = read_num(&mut j, b'\n');

        rules.push(Rule::new(name, a, b, c, d));

        line.clear();
    }

    let mut i = 0;

    while i < 4 {
        let _ = input.read_line(&mut line);
        i += 1;
    }

    line.clear();

    let check = |n: u16| rules.iter().any(|rule| rule.contains(n));

    let mut p1 = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();

        let mut n = 0;
        let mut i = 0;

        while i < bytes.len() {
            match unsafe { *bytes.get_unchecked(i) } {
                b'\n' => {
                    if !check(n) {
                        p1 += n;
                    }

                    break;
                }
                b',' => {
                    if !check(n) {
                        p1 += n;
                    }
                    n = 0;
                }
                c => n = n * 10 + (c & 0x0F) as u16,
            }

            i += 1;
        }

        if i == bytes.len() && !check(n) {
            p1 += n;
        }

        line.clear();
    }

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 485µs

    p1
}

fn part2() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut rules = Vec::with_capacity(8);

    while {
        let _ = input.read_line(&mut line);
        line.len() > 1
    } {
        let bytes = line.as_bytes();
        let mut j = 0;

        while unsafe { *bytes.get_unchecked(j) } != b':' {
            j += 1;
        }

        let name = unsafe { bytes.get_unchecked(..j) };

        let read_num = |j: &mut usize, end| {
            let mut n = 0;
            let mut byte;

            while *j < bytes.len() {
                byte = unsafe { *bytes.get_unchecked(*j) };

                if byte == end {
                    return n;
                }

                n = n * 10 + (byte & 0x0F) as u16;

                *j += 1;
            }

            n
        };

        j += 2;
        let a = read_num(&mut j, b'-');

        j += 1;
        let b = read_num(&mut j, b' ');

        j += 4;
        let c = read_num(&mut j, b'-');

        j += 1;
        let d = read_num(&mut j, b'\n');

        rules.push(Rule::new(name, a, b, c, d));

        line.clear();
    }

    let _ = input.read_line(&mut line);
    line.clear();

    let ticket = {
        let _ = input.read_line(&mut line);
        let bytes = line.as_bytes();
        let mut i = 0;
        let mut n = 0;

        let mut nums = Vec::with_capacity(rules.len());

        while i < bytes.len() {
            match bytes[i] {
                b'\n' => nums.push(n),
                b',' => {
                    nums.push(n);
                    n = 0;
                }
                c => n = n * 10 + (c & 0x0F) as u16,
            }

            i += 1;
        }

        nums
    };

    let _ = input.read_line(&mut line);
    let _ = input.read_line(&mut line);

    line.clear();

    let check = |n: u16| rules.iter().any(|rule| rule.contains(n));

    let mut possibs = HashMap::with_capacity(rules.len());

    for rule in rules.iter() {
        let mut set = HashSet::with_hasher(util::NumHasherBuilder);
        set.extend(0..rules.len() as u8);
        possibs.insert(rule, set);
    }

    let mut row = Vec::new();

    'outer: while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();

        let mut n = 0;
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'\n' => {
                    if check(n) {
                        row.push(n);
                    } else {
                        row.clear();
                        line.clear();
                        continue 'outer;
                    }
                }
                b',' => {
                    if check(n) {
                        row.push(n);
                    } else {
                        row.clear();
                        line.clear();
                        continue 'outer;
                    }

                    n = 0;
                }
                c => n = n * 10 + (c & 0x0F) as u16,
            }

            i += 1;
        }

        if i == bytes.len() {
            if check(n) {
                row.push(n);
            } else {
                row.clear();
                line.clear();
                continue 'outer;
            }
        }

        let mut i = 0;

        while i < row.len() {
            for rule in rules.iter() {
                if !rule.contains(row[i]) {
                    possibs.get_mut(rule).unwrap().remove(&(i as u8));
                }
            }

            i += 1;
        }

        row.clear();
        line.clear();
    }

    let mut possibs: Vec<_> = possibs.into_iter().collect();
    possibs.sort_by_key(|(_, i)| std::cmp::Reverse(i.len()));

    let mut p2 = 1;

    while let Some((rule, rules)) = possibs.pop() {
        let val = rules.into_iter().next().unwrap();

        for (_, rules) in possibs.iter_mut() {
            rules.remove(&val);
        }

        if rule.name.starts_with("departure") {
            p2 *= unsafe { *ticket.get_unchecked(val as usize) } as u64;
        }
    }

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 714µs

    p2
}
