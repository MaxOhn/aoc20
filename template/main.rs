use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    while matches!(input.read_line(&mut line), Ok(n) if n != 0) {
        print!("{}", line);
        line.clear();
    }
}
