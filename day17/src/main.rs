use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos3 {
    x: i8,
    y: i8,
    z: i8,
}

impl Pos3 {
    fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }
}

impl Add<(i8, i8, i8)> for Pos3 {
    type Output = Self;

    fn add(self, (x, y, z): (i8, i8, i8)) -> Self::Output {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

impl Hash for Pos3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = ((self.x as i32) << 16) + ((self.y as i32) << 8) + self.z as i32;

        n.hash(state);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos4 {
    w: i8,
    x: i8,
    y: i8,
    z: i8,
}

impl Pos4 {
    fn new(w: i8, x: i8, y: i8, z: i8) -> Self {
        Self { w, x, y, z }
    }
}

impl Add<(i8, i8, i8, i8)> for Pos4 {
    type Output = Self;

    fn add(self, (w, x, y, z): (i8, i8, i8, i8)) -> Self::Output {
        Self {
            w: self.w + w,
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

impl Hash for Pos4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = ((self.w as i64) << 24)
            + ((self.x as i64) << 16)
            + ((self.y as i64) << 8)
            + self.z as i64;

        n.hash(state);
    }
}

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 395);
    assert_eq!(p2, 2296);
}

fn part1() -> usize {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut layers = HashSet::with_capacity(256);
    let mut y = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let coords = line
            .trim_end()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| Pos3::new(x as i8, y, 0));

        layers.extend(coords);
        y += 1;
        line.clear();
    }

    let mut next_layers = HashSet::with_capacity(256);
    let mut checked = HashSet::with_capacity(512);

    for _ in 0..6 {
        for coord in layers.iter().copied() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let coord = coord + (dx, dy, dz);

                        if !checked.insert(coord) {
                            continue;
                        }

                        let mut n = 0;

                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                for dz in -1..=1 {
                                    n += layers.contains(&(coord + (dx, dy, dz))) as u16;
                                }
                            }
                        }

                        n -= layers.contains(&coord) as u16;

                        if n == 3 || (n == 2 && layers.contains(&coord)) {
                            next_layers.insert(coord);
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut layers, &mut next_layers);
        next_layers.clear();
        checked.clear();
    }

    println!("Part 1: {} [{:?}]", layers.len(), start.elapsed()); // 5ms

    layers.len()
}

fn part2() -> usize {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut layers = HashSet::with_capacity(4096);
    let mut y = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let coords = line
            .trim_end()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| Pos4::new(0, x as i8, y, 0));

        layers.extend(coords);
        y += 1;
        line.clear();
    }

    let mut next_layers = HashSet::with_capacity(4096);
    let mut checked = HashSet::with_capacity(8192);

    for _ in 0..6 {
        for coord in layers.iter().copied() {
            for dw in -1..=1 {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            let coord = coord + (dw, dx, dy, dz);

                            if !checked.insert(coord) {
                                continue;
                            }

                            let mut n = 0;

                            for dw in -1..=1 {
                                for dx in -1..=1 {
                                    for dy in -1..=1 {
                                        for dz in -1..=1 {
                                            n +=
                                                layers.contains(&(coord + (dw, dx, dy, dz))) as u16;
                                        }
                                    }
                                }
                            }

                            n -= layers.contains(&coord) as u16;

                            if n == 3 || (n == 2 && layers.contains(&coord)) {
                                next_layers.insert(coord);
                            }
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut layers, &mut next_layers);
        next_layers.clear();
        checked.clear();
    }

    let p2 = layers.len();

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 170ms

    p2
}
