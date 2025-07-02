use std::collections::{HashMap, HashSet};

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

    for e in &expenses {
        if seen.contains(&(2020 - e)) {
            mul = Some(e * (2020 - e));
            break;
        }
        seen.insert(e);
    }

    println!("  │  ├─ Part 1: {}", mul.unwrap());

    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut res = None;
    let size = expenses.len();

    for i in 0..size {
        for j in i + 1..size {
            let diff = 2020 - (expenses[i] + expenses[j]) as i32;
            if let Some(mul) = map.get(&expenses[j]) {
                res = Some(mul * expenses[j]);
                break;
            }

            if diff >= 0 {
                map.insert(diff as u32, expenses[i] * expenses[j]);
            }
        }
    }

    println!("  │  └─ Part 2: {}", res.unwrap());
}
