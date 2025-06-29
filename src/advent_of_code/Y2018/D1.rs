use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Day 1 - Chronal Calibration");

    let path = "input/advent_of_code/Y2018/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let changes = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("  │  ├─ Part 1: {}", changes.iter().sum::<i32>());

    let mut twice = None;
    let mut sum = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    let mut i = 0;
    loop {
        sum += changes[i % changes.len()];
        if seen.contains(&sum) {
            twice = Some(sum);
            break;
        }
        seen.insert(sum);
        i += 1;
    }

    println!("  │  └─ Part 2: {}", twice.unwrap());
}
