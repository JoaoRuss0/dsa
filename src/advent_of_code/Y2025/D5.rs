pub fn run() {
    println!("  ├─ Day 5 - Cafeteria");

    let path = "input/advent_of_code/Y2025/D5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (ranges_s, available_s) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges_s.lines().map(Range::from).collect::<Vec<Range>>();
    ranges.sort_by_key(|r| r.min);

    let mut merged: Vec<Range> = Vec::new();
    let mut new_range = ranges[0];

    let mut i = 1;
    while i < ranges.len() {
        if new_range.collide(&ranges[i]) {
            new_range = new_range.merge(&ranges[i]);
            i += 1;
            continue;
        }
        merged.push(new_range);
        new_range = ranges[i];
        i += 1;
    }
    merged.push(new_range);
    ranges = merged;

    let available = available_s
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut fresh = 0;
    'is_fresh: for a in available {
        for r in ranges.iter() {
            if r.min <= a && r.max >= a {
                fresh += 1;
                continue 'is_fresh;
            }
        }
    }

    println!("  │  ├─ Part 1: {}", fresh);
    println!(
        "  │  └─ Part 2: {}",
        ranges.iter().map(|r| r.max - r.min + 1).sum::<usize>()
    );
}

#[derive(Clone, Copy)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn from(line: &str) -> Self {
        let (min, max) = line.split_once("-").unwrap();
        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }

    fn collide(&self, other: &Self) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}
