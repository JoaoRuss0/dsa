pub fn run() {
    println!("  ├─ Day 5 - Cafeteria");

    let path = "input/advent_of_code/Y2025/D5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (ranges_s, available_s) = input.split_once("\n\n").unwrap();
    let ranges = ranges_s.lines().map(Range::from).collect::<Vec<Range>>();
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
    //println!("  │  └─ Part 2: {}", 0);
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
}
