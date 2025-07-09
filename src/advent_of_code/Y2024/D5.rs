use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("  ├─ Day 5 - Print Queue");

    let path = "input/advent_of_code/Y2024/D5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut rules = HashMap::new();

    sections[0].lines().for_each(|l| {
        let (prev, next) = l.split_once("|").unwrap();
        rules
            .entry(prev.parse::<usize>().unwrap())
            .or_insert(HashSet::new())
            .insert(next.parse::<usize>().unwrap());
    });

    let updates = sections[1]
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let correct = updates
        .into_iter()
        .filter(|u| {
            for i in 0..u.len() - 1 {
                for j in i..u.len() {
                    if let Some(after_next) = rules.get(&u[j]) {
                        if after_next.contains(&u[i]) {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect::<Vec<Vec<usize>>>();

    println!(
        "  │  ├─ Part 1: {}",
        correct.iter().map(|u| u[u.len() / 2]).sum::<usize>()
    );
    //println!("  │  └─ Part 2: {}", );
}
