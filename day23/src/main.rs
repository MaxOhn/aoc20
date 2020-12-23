use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 27_865_934);
    assert_eq!(p2, 170_836_011_000);
}

fn part1() -> u32 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let _ = input.read_line(&mut line);

    let mut n: u32 = util::Parse::parse(line.as_bytes());
    let mut circle = Vec::with_capacity(9);

    while n > 0 {
        circle.push(((n % 10) - 1) as u8);
        n /= 10;
    }

    circle.reverse();

    const LEN: usize = 9;

    let mut i = 0;

    for _m in 0..100 {
        // println!("-- move {} --", _m + 1);
        // println!("{:?}", circle);

        let target = (circle[i] + 8) % 9;
        let destination = circle.iter().position(|&n| n == target).unwrap();
        let distance = (9 + destination - i) % 9;

        if distance <= 3 {
            let target = (circle[i] + 7) % 9;
            let destination = circle.iter().position(|&n| n == target).unwrap();
            let distance = (9 + destination - i) % 9;

            if distance <= 3 {
                let target = (circle[i] + 6) % 9;
                let destination = circle.iter().position(|&n| n == target).unwrap();
                let distance = (9 + destination - i) % 9;

                if distance <= 3 {
                    let target = (circle[i] + 5) % 9;
                    let destination = circle.iter().position(|&n| n == target).unwrap();
                    let distance = (9 + destination - i) % 9;

                    if destination > i {
                        circle[i + 1..destination + 1].rotate_left(3);
                    } else {
                        let mut j = 0;

                        while j + 6 < distance {
                            circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                            circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                            circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                            j += 3;
                        }

                        for l in 0..distance - j - 3 {
                            circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                            circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                            circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
                        }
                    }
                } else if destination > i {
                    circle[i + 1..destination + 1].rotate_left(3);
                } else {
                    let mut j = 0;

                    while j + 6 < distance {
                        circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                        circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                        circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                        j += 3;
                    }

                    for l in 0..distance - j - 3 {
                        circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                        circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                        circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
                    }
                }
            } else if destination > i {
                circle[i + 1..destination + 1].rotate_left(3);
            } else {
                let mut j = 0;

                while j + 6 < distance {
                    circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                    circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                    circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                    j += 3;
                }

                for l in 0..distance - j - 3 {
                    circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                    circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                    circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
                }
            }
        } else if destination > i {
            circle[i + 1..destination + 1].rotate_left(3);
        } else {
            let mut j = 0;

            while j + 6 < distance {
                circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                j += 3;
            }

            for l in 0..distance - j - 3 {
                circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
            }
        }

        i = (i + 1) % 9;
    }

    let p1 = circle
        .into_iter()
        .cycle()
        .skip_while(|&n| n != 0)
        .skip(1)
        .map(|n| n as u32 + 1)
        .take(8)
        .fold(0, |num, n| num * 10 + n);

    println!("Part 1: {} {:?}", p1, start.elapsed()); // 110µs

    p1
}

fn part2() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let _ = input.read_line(&mut line);

    let mut n: u32 = util::Parse::parse(line.as_bytes());
    let mut circle = Vec::with_capacity(1_000_000);

    while n > 0 {
        circle.push((n % 10) - 1);
        n /= 10;
    }

    circle.reverse();

    circle.extend(9..1_000_000);

    const LEN: usize = 1_000_000;
    const LEN_U: u32 = LEN as u32;

    let mut i = 0;

    for m in 0..10_000_000 {
        if m % 10_000 == 0 {
            println!("m={} [{:?}]", m, start.elapsed());
        }

        let target = (circle[i] + LEN_U - 1) % LEN_U;
        let destination = circle.iter().position(|&n| n == target).unwrap();
        let distance = (LEN + destination - i) % LEN;

        if distance <= 3 {
            let target = (circle[i] + LEN_U - 2) % LEN_U;
            let destination = circle.iter().position(|&n| n == target).unwrap();
            let distance = (LEN + destination - i) % LEN;

            if distance <= 3 {
                let target = (circle[i] + LEN_U - 3) % LEN_U;
                let destination = circle.iter().position(|&n| n == target).unwrap();
                let distance = (LEN + destination - i) % LEN;

                if distance <= 3 {
                    let target = (circle[i] + LEN_U - 4) % LEN_U;
                    let destination = circle.iter().position(|&n| n == target).unwrap();
                    let distance = (LEN + destination - i) % LEN;

                    if destination > i {
                        circle[i + 1..destination + 1].rotate_left(3);
                    } else {
                        let mut j = 0;

                        while j + 6 < distance {
                            circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                            circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                            circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                            j += 3;
                        }

                        let k = i + j;

                        for l in 0..distance - j - 3 {
                            circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                            circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                            circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
                        }
                    }
                } else if destination > i {
                    circle[i + 1..destination + 1].rotate_left(3);
                } else {
                    let mut j = 0;

                    while j + 6 < distance {
                        circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                        circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                        circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                        j += 3;
                    }

                    let k = i + j;

                    for l in 0..distance - j - 3 {
                        circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                        circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                        circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
                    }
                }
            } else if destination > i {
                circle[i + 1..destination + 1].rotate_left(3);
            } else {
                let mut j = 0;

                while j + 6 < distance {
                    circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                    circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                    circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                    j += 3;
                }

                let k = i + j;

                for l in 0..distance - j - 3 {
                    circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                    circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                    circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
                }
            }
        } else if destination > i {
            circle[i + 1..destination + 1].rotate_left(3);
        } else {
            let mut j = 0;

            while j + 6 < distance {
                circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                j += 3;
            }

            let k = i + j;

            for l in 0..distance - j - 3 {
                circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
            }
        }

        i = (i + 1) % LEN;
    }

    let p2 = circle
        .into_iter()
        .cycle()
        .skip_while(|&n| n != 0)
        .skip(1)
        .map(|n| n as u32 + 1)
        .take(2)
        .fold(1, |prod, cup| prod * cup as u64);

    println!("Part 2: {} {:?}", p2, start.elapsed()); // 3 hours

    p2
}
