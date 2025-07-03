pub fn run() {
    println!("  ├─ Day 1 - Sonar Sweep");

    let path = "input/advent_of_code/Y2021/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let depths = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut increases = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            increases += 1;
        }
    }

    println!("  │  ├─ Part 1: {increases}");

    increases = 0;
    let mut last = depths[0] + depths[1] + depths[2];
    for window in depths[1..].windows(3) {
        let sum = window[0] + window[1] + window[2];
        if sum > last {
            increases += 1;
        }
        last = sum;
    }

    println!("  │  └─ Part 2: {increases}");
}
