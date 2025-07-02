use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Day 1 - Report Repair");

    let path = "input/advent_of_code/Y2020/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let expenses = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut seen = HashSet::new();
    let mut mul = None;

    for e in expenses {
        if seen.contains(&(2020 - e)) {
            mul = Some(e * (2020 - e));
            break;
        }
        seen.insert(e);
    }

    println!("  │  ├─ Part 1: {}", mul.unwrap());
    //println!("  │  └─ Part 2: {}", );
}
