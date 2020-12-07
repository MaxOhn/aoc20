use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::time::Instant;

type Bags = HashMap<Key, Vec<BagAmount>>;
type CachePart1 = HashMap<Key, bool>;
type CachePart2 = HashMap<Key, usize>;

const MY_BAG: &str = "shiny gold";

fn main() {
    let start = Instant::now();
    let file = std::fs::File::open("./input").unwrap();
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut names = HashMap::with_capacity(256);
    let mut bags = HashMap::with_capacity(256);

    while input.read_line(&mut line).unwrap() != 0 {
        let bytes = line.as_bytes();

        let i = bag_end_idx(bytes);

        let outer = if let Some(bag) = names.get(unsafe { line.get_unchecked(..i) }) {
            Rc::clone(bag)
        } else {
            let bag = Rc::new(unsafe { line.get_unchecked(..i) }.to_owned());
            names.insert(Key(Rc::clone(&bag)), Rc::clone(&bag));

            bag
        };

        if unsafe { line.get_unchecked(i + 14..) }.starts_with("no ") {
            bags.entry(Key(outer)).or_insert_with(Vec::new);
            line.clear();
            continue;
        }

        for inner in unsafe { line.get_unchecked(i + 14..) }.split(", ") {
            let bytes = inner.as_bytes();
            let n = unsafe { *bytes.get_unchecked(0) } - b'0';

            let bytes = match unsafe { inner.get_unchecked(2..) }.find('.') {
                Some(idx) => unsafe { bytes.get_unchecked(2..idx) },
                None => unsafe { bytes.get_unchecked(2..) },
            };

            let i = bag_end_idx(bytes);

            let inner = if let Some(bag) = names.get(unsafe { inner.get_unchecked(2..i + 2) }) {
                Rc::clone(bag)
            } else {
                let bag = Rc::new(unsafe { inner.get_unchecked(2..i + 2).to_owned() });
                names.insert(Key(Rc::clone(&bag)), Rc::clone(&bag));

                bag
            };

            let inner = BagAmount::new(n, inner);

            bags.entry(Key(Rc::clone(&outer)))
                .or_insert_with(|| Vec::with_capacity(1))
                .push(inner);
        }

        line.clear();
    }

    println!("Setup: {:?}", start.elapsed()); // 950µs

    let start = Instant::now();
    let mut cache = HashMap::new();

    let p1 = bags
        .keys()
        .filter(|bag| contains_recursive(&bag.0, &bags, &mut cache))
        .count();

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 250µs

    let start = Instant::now();
    let mut cache = HashMap::new();

    let p2: usize = bags
        .get(MY_BAG)
        .unwrap()
        .iter()
        .map(|BagAmount { amount, bag }| *amount as usize * count_recursive(bag, &bags, &mut cache))
        .sum();

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 9µs

    assert_eq!(p1, 161);
    assert_eq!(p2, 30_899);
}

fn contains_recursive(bag: &Rc<String>, bags: &Bags, cache: &mut CachePart1) -> bool {
    let key = Key(Rc::clone(bag));

    if let Some(value) = cache.get(bag) {
        return *value;
    } else if bag.as_ref() == MY_BAG {
        return false;
    }

    let inner = bags.get(bag).unwrap();

    if inner.iter().any(|bag| bag.bag.as_ref() == MY_BAG) {
        cache.insert(key, true);
        return true;
    }

    for BagAmount { bag, .. } in inner {
        if contains_recursive(bag, bags, cache) {
            cache.insert(key, true);
            return true;
        }
    }

    cache.insert(key, false);
    false
}

fn count_recursive(bag: &Rc<String>, bags: &Bags, cache: &mut CachePart2) -> usize {
    let key = Key(Rc::clone(bag));

    if let Some(value) = cache.get(bag) {
        return *value;
    }

    let mut count = 1;

    for BagAmount { amount, bag } in bags.get(bag).unwrap() {
        count += *amount as usize * count_recursive(bag, bags, cache);
    }

    cache.insert(key, count);
    count
}

struct BagAmount {
    amount: u8,
    bag: Rc<String>,
}

impl BagAmount {
    fn new(amount: u8, bag: Rc<String>) -> Self {
        Self { amount, bag }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Key(Rc<String>);

impl Borrow<str> for Key {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<Rc<String>> for Key {
    fn borrow(&self) -> &Rc<String> {
        &self.0
    }
}

fn bag_end_idx(bag: &[u8]) -> usize {
    let mut i = 0;
    let mut found_first = false;

    loop {
        if unsafe { *bag.get_unchecked(i) } == b' ' {
            if found_first {
                return i;
            }

            found_first = true;
        }

        i += 1;
    }
}
