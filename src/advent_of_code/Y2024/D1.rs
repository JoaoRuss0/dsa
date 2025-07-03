pub fn run() {
    println!("  ├─ Day 1 - Historian Hysteria");

    let path = "input/advent_of_code/Y2024/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let diffs = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>();

    println!("  │  ├─ Part 1: {}", diffs);
    //println!("  │  └─ Part 2: {}", );
}
