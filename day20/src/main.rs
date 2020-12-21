use hashbrown::HashMap;
use std::fmt;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::time::Instant;

macro_rules! get {
    ($bytes:ident, $i:expr) => {
        unsafe { *$bytes.get_unchecked($i) }
    };
}

#[derive(Copy, Clone, Debug)]
enum Side {
    Top,
    Right,
    Bot,
    Left,
}

#[derive(Clone)]
struct Tile {
    id: u16,
    w: usize,
    tile: Vec<u8>,
    borders: [u16; 4],
}

impl Tile {
    fn new(id: u16, w: usize) -> Self {
        Self {
            id,
            w,
            tile: Vec::with_capacity(w * w),
            borders: [0; 4],
        }
    }

    fn borders(&mut self) {
        // for &c in &self.tile[..self.w] {
        //     self.borders[0] <<= 1;
        //     self.borders[0] += (c == b'#') as u16;
        // }

        // for y in 1..=self.w {
        //     self.borders[1] <<= 1;
        //     self.borders[1] += (self[y * self.w - 1] == b'#') as u16;
        // }

        // for &c in self.tile[self.w * self.w - self.w..].iter().rev() {
        //     self.borders[2] <<= 1;
        //     self.borders[2] += (c == b'#') as u16;
        // }

        // for y in (0..self.w).rev() {
        //     self.borders[3] <<= 1;
        //     self.borders[3] += (self[y * self.w] == b'#') as u16;
        // }
    }

    fn top(&self) -> u16 {
        let mut edge = 0;

        for &c in &self.tile[..self.w] {
            edge <<= 1;
            edge += (c == b'#') as u16;
        }

        edge
    }

    fn right(&self) -> u16 {
        let mut edge = 0;

        for y in 1..=self.w {
            edge <<= 1;
            edge += (self[y * self.w - 1] == b'#') as u16;
        }

        edge
    }

    fn bot(&self) -> u16 {
        let mut edge = 0;

        for &c in self.tile[self.w * self.w - self.w..].iter().rev() {
            edge <<= 1;
            edge += (c == b'#') as u16;
        }

        edge
    }

    fn left(&self) -> u16 {
        let mut edge = 0;

        for y in (0..self.w).rev() {
            edge <<= 1;
            edge += (self[y * self.w] == b'#') as u16;
        }

        edge
    }

    fn fits(&self, edge: u16, side: Side) -> bool {
        match side {
            Side::Top => edge == (self.top().reverse_bits() >> 6),
            Side::Right => edge == (self.right().reverse_bits() >> 6),
            Side::Bot => edge == (self.bot().reverse_bits() >> 6),
            Side::Left => edge == (self.left().reverse_bits() >> 6),
        }

        // match side {
        //     Side::Top => edge == self.top(),
        //     Side::Right => edge == self.right(),
        //     Side::Bot => edge == self.bot(),
        //     Side::Left => edge == self.left(),
        // }
    }

    // fn top(&self) -> u16 {
    //     self.borders[0]
    // }

    // fn right(&self) -> u16 {
    //     self.borders[1]
    // }

    // fn bot(&self) -> u16 {
    //     self.borders[2]
    // }

    // fn left(&self) -> u16 {
    //     self.borders[3]
    // }

    fn any(&self, border: u16) -> Option<Side> {
        if self.fits(border, Side::Top) {
            return Some(Side::Top);
        }

        if self.fits(border, Side::Right) {
            return Some(Side::Right);
        }

        if self.fits(border, Side::Bot) {
            return Some(Side::Bot);
        }

        if self.fits(border, Side::Left) {
            return Some(Side::Left);
        }

        None
    }

    fn flip_horizontal(&mut self) {
        let w = self.w;

        for x in 0..w {
            for y in 0..w / 2 {
                self.swap(x + w * y, x + w * (w - y - 1));
            }
        }

        // self.borders[1] = self.borders[1].reverse_bits() >> (16 - w);
        // self.borders[3] = self.borders[3].reverse_bits() >> (16 - w);
        // self.borders.swap(0, 2);
    }

    fn flip_vertical(&mut self) {
        let w = self.w;

        for x in 0..w / 2 {
            for y in 0..self.len() / self.w {
                self.swap(x + w * y, w - x - 1 + w * y);
            }
        }

        // self.borders[0] = self.borders[0].reverse_bits() >> (16 - w);
        // self.borders[2] = self.borders[2].reverse_bits() >> (16 - w);
        // self.borders.swap(1, 3);
    }

    fn transpose(&mut self) {
        let w = self.w;

        for x in 0..w {
            for y in x..w {
                self.swap(x + w * y, y + w * x);
            }
        }

        for i in 0..4 {
            self.borders[i] = self.borders[i].reverse_bits() >> (16 - w);
        }

        // self.borders.swap(0, 3);
        // self.borders.swap(1, 2);
    }

    fn rotate_cw(&mut self) {
        self.transpose();
        self.flip_vertical();
    }

    #[allow(dead_code)]
    fn rotate_ccw(&mut self) {
        self.transpose();
        self.flip_horizontal();
    }
}

impl Deref for Tile {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.tile
    }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tile
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile {}:\n", self.id)?;
        let mut rows = self.chunks(self.w);

        if let Some(first) = rows.next() {
            for &c in first {
                write!(f, "{}", c as char)?;
            }

            for row in rows {
                f.write_str("\n")?;

                for &c in row {
                    write!(f, "{}", c as char)?;
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let p1 = part1();
    let _p2 = part2();

    assert_eq!(p1, 29_125_888_761_511);
    // asser_eq!(p2, 0);
}

fn part2() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let w;
    let mut tiles = Vec::with_capacity(128);
    let mut edges = HashMap::with_capacity(1024);

    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();
    let mut id = 0;
    let mut i = 5;

    loop {
        match get!(bytes, i) {
            b':' => break,
            digit => id = id * 10 + (digit & 0x0F) as u16,
        }

        i += 1;
    }

    line.clear();
    let _ = input.read_line(&mut line);

    w = line.len() - 1;

    let mut tile = Tile::new(id, w);
    tile.extend(&line.as_bytes()[..w]);

    loop {
        line.clear();
        let _ = input.read_line(&mut line);

        if line.len() == 1 {
            break;
        }

        tile.extend(&line.as_bytes()[..w]);
    }

    tile.borders();

    // println!("{}", tile);
    // for border in &tile.borders {
    //     println!("{:0>10b}", *border);
    // }
    // println!("\nTransposing...\n");
    // tile.transpose();
    // println!("{}", tile);

    // for border in &tile.borders {
    //     println!("{:0>10b}", *border);
    // }

    // return 3;

    count_edges(&tile, &mut edges);
    tiles.push(tile);

    line.clear();

    loop {
        let _ = input.read_line(&mut line);

        if line.is_empty() {
            break;
        }

        let bytes = line.as_bytes();
        let mut id = 0;
        let mut i = 5;

        loop {
            match get!(bytes, i) {
                b':' => break,
                digit => id = id * 10 + (digit & 0x0F) as u16,
            }

            i += 1;
        }

        let mut tile = Tile::new(id, w);

        while {
            line.clear();

            let read = input
                .read_line(&mut line)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() });

            read > 0 && line.len() > 1
        } {
            tile.extend(&line.as_bytes()[..w]);
        }

        tile.borders();
        count_edges(&tile, &mut edges);
        tiles.push(tile);

        line.clear();
    }

    // for (edge, count) in edges {
    //     println!("{:0>len$b}: {}", edge, count, len = w);
    // }

    let w_outer = (tiles.len() as f32).sqrt() as usize;
    let mut i = tiles.len() - 1;

    let mut corner = loop {
        let tile = &mut tiles[i];
        if valid_corner_tile(&tile, &edges) {
            break tiles.swap_remove(i);
        } else {
            tile.transpose();

            if valid_corner_tile(&tile, &edges) {
                break tiles.swap_remove(i);
            }
        }

        i -= 1;
    };

    while edges[&corner.left()] != 1 || edges[&corner.top()] != 1 {
        corner.rotate_cw();
    }

    let mut picture = vec![Vec::with_capacity(w_outer); w_outer];
    picture[0].push(corner);

    recurse((0, 0), &mut tiles, &mut picture);

    for (i, row) in picture.iter().enumerate() {
        println!("Row {}", i);

        for tile in row {
            println!("{}", tile);
            println!();
        }
    }

    for row in &picture {
        println!("Len: {}", row.len());
    }

    println!("Valid: {}", check_valid(&picture, w_outer, w));

    println!("Part 2: {} [{:?}]", 0, start.elapsed()); //

    todo!()
}

fn recurse((x, y): (usize, usize), tiles: &mut Vec<Tile>, picture: &mut [Vec<Tile>]) {
    // match picture[y].get(x) {
    //     Some(t) => println!("!! Checking ({},{}) !!\n{}", x, y, t),
    //     None => return,
    // }
    if x > 0 && y < picture.len() - 1 && picture.last().unwrap().len() > x {
        // assert!(
        //     picture[y + 1].len() > x,
        //     "There should be a bot neighbor already"
        // );
    }
    if y + 1 < picture.len() && x >= picture[y + 1].len() {
        let (bot_neighbor, from_top) = {
            let tile = &picture[y][x];
            // println!("{},{}:\n{}", x, y, tile);
            let from_top = tile.bot();

            let bot_neighbor = tiles
                .iter()
                .position(|tile| {
                    // println!("{}", tile);
                    // println!("Bot: {:0>10b}", tile.bot());

                    tile.any(from_top).is_some()
                })
                .or_else(|| {
                    tiles.iter_mut().position(|tile| {
                        // println!("Before");
                        // println!("{}", tile);
                        tile.transpose();

                        // println!("After");
                        // println!("{}", tile);
                        // println!("Bot: {:0>10b}", tile.bot());

                        tile.any(from_top).is_some()
                    })
                });

            (bot_neighbor, from_top)
        };

        if let Some(bot_neighbor) = bot_neighbor {
            let mut bot_neighbor = tiles.swap_remove(bot_neighbor);

            // println!("\nBot neighbor:\n{}", bot_neighbor);
            // println!("Bot: {:0>10b}", bot_neighbor.bot());

            // println!("Searching for {:0>10b} on top side\n", from_top);

            let rev = from_top.reverse_bits() >> 6;

            while bot_neighbor.top() != rev {
                // println!("Top: {:0>10b}", bot_neighbor.top());
                bot_neighbor.rotate_cw();
            }

            picture[y + 1].push(bot_neighbor);

            recurse((x, y + 1), tiles, picture);
        } else {
            // println!("[a] No bot neighbor for {}", picture[y][x].id);
        }
    } else {
        // println!("[b] No bot neighbor for {}", picture[y][x].id);
    }

    let (right_neighbor, from_left) = {
        let tile = &picture[y][x];
        let from_left = tile.right();

        // println!("Reminder:");
        // println!("{}", tile);
        // println!("from_left: {:0>10b}", from_left);

        let from_bot = if y + 1 < picture.len() {
            picture[y + 1].get(x + 1).map(Tile::bot)
        } else {
            None
        };
        // 0100101100

        let right_neighbor = tiles
            .iter()
            .position(|tile| {
                // println!("{}", tile);

                let side = match tile.any(from_left) {
                    Some(side) => side,
                    None => return false,
                };

                if let Some(from_bot) = from_bot {
                    // println!("-----a----");
                    // println!("{}", tile);
                    // println!("Left: {:0>10b} | Bot: {:0>10b}", from_left, from_bot);
                    let res = match side {
                        Side::Top => from_bot == tile.right(),
                        Side::Right => from_bot == tile.bot(),
                        Side::Bot => from_bot == tile.left(),
                        Side::Left => from_bot == tile.top(),
                    };
                    // println!("Good [{:?}]? {}", side, res);
                    return res;
                }

                true
            })
            .or_else(|| {
                tiles.iter_mut().position(|tile| {
                    // println!("Before:");
                    // println!("{}", tile);

                    tile.transpose();

                    // println!("After:");
                    // println!("{}", tile);
                    // println!("Bot: {:0>10b}", tile.bot());

                    let side = match tile.any(from_left) {
                        Some(side) => side,
                        None => return false,
                    };

                    if let Some(from_bot) = from_bot {
                        // println!("-----b----");
                        // println!("{}", tile);
                        // println!("Left: {:0>10b} | Bot: {:0>10b}", from_left, from_bot);
                        let res = match side {
                            Side::Top => from_bot == tile.right(),
                            Side::Right => from_bot == tile.bot(),
                            Side::Bot => from_bot == tile.left(),
                            Side::Left => from_bot == tile.top(),
                        };
                        // println!("Good [{:?}]? {}", side, res);
                        return res;
                    }

                    true
                })
            });

        (right_neighbor, from_left)
    };

    if let Some(right_neighbor) = right_neighbor {
        let mut right_neighbor = tiles.swap_remove(right_neighbor);

        // println!("\nRight neighbor:\n{}", right_neighbor);
        // println!("Bot: {:0>10b}", right_neighbor.bot());

        // println!("Searching for {:0>10b} on right side\n", from_left);

        let rev = from_left.reverse_bits() >> 6;

        while right_neighbor.left() != rev {
            right_neighbor.rotate_cw();
        }

        picture[y].push(right_neighbor);

        recurse((x + 1, y), tiles, picture);
    } else {
        // println!("No right neighbor for {}", picture[y][x].id);
    }

    // println!("Done with {}", picture[y][x].id);
}

#[rustfmt::skip]




























fn part1() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let w;
    let mut tiles = Vec::with_capacity(16);
    let mut edges = HashMap::with_capacity(1024);

    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();
    let mut id = 0;
    let mut i = 5;

    loop {
        match get!(bytes, i) {
            b':' => break,
            digit => id = id * 10 + (digit & 0x0F) as u16,
        }

        i += 1;
    }

    line.clear();
    let _ = input.read_line(&mut line);

    w = line.len() - 1;

    let mut tile = Tile::new(id, w);
    tile.extend(&line.as_bytes()[..w]);

    loop {
        line.clear();
        let _ = input.read_line(&mut line);

        if line.len() == 1 {
            break;
        }

        tile.extend(&line.as_bytes()[..w]);
    }

    tile.borders();
    count_edges(&tile, &mut edges);
    tiles.push(tile);

    line.clear();

    loop {
        let _ = input.read_line(&mut line);

        if line.is_empty() {
            break;
        }

        let bytes = line.as_bytes();
        let mut id = 0;
        let mut i = 5;

        loop {
            match get!(bytes, i) {
                b':' => break,
                digit => id = id * 10 + (digit & 0x0F) as u16,
            }

            i += 1;
        }

        let mut tile = Tile::new(id, w);

        while {
            line.clear();

            let read = input
                .read_line(&mut line)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() });

            read > 0 && line.len() > 1
        } {
            tile.extend(&line.as_bytes()[..w]);
        }

        tile.borders();
        count_edges(&tile, &mut edges);
        tiles.push(tile);

        line.clear();
    }

    let mut p1 = 1;

    for mut tile in tiles {
        if valid_corner_tile(&tile, &edges) {
            p1 *= tile.id as u64;
            continue;
        }

        tile.transpose();

        if valid_corner_tile(&tile, &edges) {
            p1 *= tile.id as u64;
        }
    }

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 256µs

    p1
}

fn valid_corner_tile(tile: &Tile, edges: &HashMap<u16, u8>) -> bool {
    (edges[&tile.top()] == 1) as u8
        + (edges[&tile.bot()] == 1) as u8
        + (edges[&tile.left()] == 1) as u8
        + (edges[&tile.right()] == 1) as u8
        == 2
}

fn count_edges(tile: &Tile, edges: &mut HashMap<u16, u8>) {
    let shift = 16 - tile.w;

    let edge = tile.top();
    *edges.entry(edge).or_insert(0) += 1;
    *edges.entry(edge.reverse_bits() >> shift).or_insert(0) += 1;

    let edge = tile.bot();
    *edges.entry(edge).or_insert(0) += 1;
    *edges.entry(edge.reverse_bits() >> shift).or_insert(0) += 1;

    let edge = tile.left();
    *edges.entry(edge).or_insert(0) += 1;
    *edges.entry(edge.reverse_bits() >> shift).or_insert(0) += 1;

    let edge = tile.right();
    *edges.entry(edge).or_insert(0) += 1;
    *edges.entry(edge.reverse_bits() >> shift).or_insert(0) += 1;
}

#[allow(dead_code)]
fn check_valid(tiles: &[Vec<Tile>], w_outer: usize, w_inner: usize) -> bool {
    for x in 0..w_outer - 1 {
        for y in 0..w_outer {
            let left = &tiles[y][x];
            let right = &tiles[y][x + 1];

            for y_inner in 0..w_inner {
                if left[w_inner - 1 + w_inner * y_inner] != right[w_inner * y_inner] {
                    return false;
                }
            }
        }
    }

    let start = w_inner * w_inner - w_inner;

    for x in 0..w_outer {
        for y in 0..w_outer - 1 {
            let top = &tiles[y][x];
            let bot = &tiles[y + 1][x];

            for x_inner in 0..w_inner {
                if top[start + x_inner] != bot[x_inner] {
                    return false;
                }
            }
        }
    }

    true
}

fn _main() {
    let mut tiles = Vec::with_capacity(4);
    let mut tile = Tile::new(1, 2);
    tile.extend(&[49, 50]);
    tile.extend(&[51, 52]);
    tiles.push(tile);

    let mut tile = Tile::new(2, 2);
    tile.extend(&[50, 53]);
    tile.extend(&[52, 54]);
    tiles.push(tile);

    let mut tile = Tile::new(3, 2);
    tile.extend(&[51, 52]);
    tile.extend(&[55, 56]);
    tiles.push(tile);

    let mut tile = Tile::new(4, 2);
    tile.extend(&[52, 54]);
    tile.extend(&[56, 57]);
    tiles.push(tile);

    for tile in tiles.iter() {
        println!("{}\n", tile);
    }

    // println!("Valid: {}", check_valid(&tiles, 2, 2));

    // let mut tile = Tile::new(1, 4);
    // tile.extend(&[b'a', b'b', b'c', b'd']);
    // tile.extend(&[b'e', b'f', b'g', b'h']);
    // tile.extend(&[b'i', b'j', b'k', b'l']);
    // tile.extend(&[b'm', b'n', b'o', b'p']);

    // println!("{}", tile);

    // tile.transpose();
    // println!("\nTransposing...\n");
    // println!("{}", tile);

    // tile.rotate_cw();
    // println!("\nRotating cw...\n");
    // println!("{}", tile);

    // tile.rotate_ccw();
    // println!("\nRotating ccw...\n");
    // println!("{}", tile);

    // tile.flip_vertical();
    // println!("\nFlipping vertically...\n");
    // println!("{}", tile);

    // tile.flip_horizontal();
    // println!("\nFlipping horizontally...\n");
    // println!("{}", tile);
}
