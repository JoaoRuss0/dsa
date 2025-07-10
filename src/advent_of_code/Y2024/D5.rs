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

    let (correct, incorrect): (Vec<Vec<usize>>, Vec<Vec<usize>>) = updates
        .into_iter()
        .partition(|u| correctly_ordered(u, &rules));

    println!("  │  ├─ Part 1: {}", middles(correct));

    let corrected = incorrect
        .iter()
        .map(|u| order(u.to_vec(), &rules))
        .collect();

    println!("  │  └─ Part 2: {}", middles(corrected));
}

fn middles(updates: Vec<Vec<usize>>) -> usize {
    updates.iter().map(|u| u[u.len() / 2]).sum::<usize>()
}

fn correctly_ordered(update: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    for i in 0..update.len() - 1 {
        for j in i..update.len() {
            if let Some(after_next) = rules.get(&update[j]) {
                if after_next.contains(&update[i]) {
                    return false;
                }
            }
        }
    }

    true
}

fn order(update: Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut update = update;

    let mut i = 0;

    while i < update.len() - 1 {
        let mut j = i + 1;
        let mut re_check = false;
        while j < update.len() {
            if let Some(after_next) = rules.get(&update[j]) {
                if after_next.contains(&update[i]) {
                    update.swap(i, j);
                    re_check = true;
                    break;
                }
            }
            j += 1;
        }

        if !re_check {
            i += 1;
        }
    }

    update
}
