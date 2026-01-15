use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

pub fn run() {
    println!("  ├─ Puzzle 3: Bush Salesman");

    let path = "input/flip_flop/Y2025/P3.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let bushes: Vec<RGB> = input
        .lines()
        .map(|l| {
            let split = l.split(",").collect::<Vec<&str>>();
            RGB::new(split[0], split[1], split[2])
        })
        .collect();

    let mut counts = HashMap::new();
    bushes.iter().for_each(|b| {
        *counts.entry(b).or_insert(0) += 1;
    });

    println!(
        "  │  ├─ Part 1: {}",
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .unwrap()
            .0
    );
    //println!("  │  ├─ Part 2: {}",);
    //println!("  │  └─ Part 3: {}",);
}

#[derive(Eq, Debug)]
struct RGB {
    r: u16,
    g: u16,
    b: u16,
}

impl RGB {
    fn new(r: &str, g: &str, b: &str) -> Self {
        Self {
            r: r.parse::<u16>().unwrap(),
            g: g.parse::<u16>().unwrap(),
            b: b.parse::<u16>().unwrap(),
        }
    }
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Hash for RGB {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.r, self.g, self.b)
    }
}
