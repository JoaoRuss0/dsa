use std::collections::{HashMap, VecDeque};

pub fn run() {
    println!("  ├─ Problem 9 - Windy Bargain");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem9.txt").unwrap();

    let mut sections = input.split("\n\n").collect::<VecDeque<&str>>();

    let mut balances: HashMap<&str, i64> = HashMap::new();
    sections
        .pop_front()
        .unwrap()
        .lines()
        .map(|b| b.split_once(" HAS ").unwrap())
        .for_each(|(name, balance)| {
            balances.insert(name, balance.parse::<i64>().unwrap());
        });

    let transactions: Vec<(&str, &str, i64)> = sections
        .pop_back()
        .unwrap()
        .lines()
        .map(|b| b.split_whitespace().collect::<Vec<&str>>())
        .map(|s| (s[1], s[3], s[5].parse::<i64>().unwrap()))
        .collect();

    let mut applied = balances.clone();
    transactions.iter().for_each(|&(from, to, amount)| {
        *applied.get_mut(from).unwrap() -= amount;
        *applied.get_mut(to).unwrap() += amount;
    });

    let mut values = applied.values().map(|v| *v).collect::<Vec<i64>>();
    values.sort();
    println!(
        "  │  ├─ Part 1: {:?}",
        values.iter().rev().take(3).sum::<i64>()
    );

    applied = balances.clone();
    transactions.iter().for_each(|&(from, to, amount)| {
        let available = *applied.get(from).unwrap();
        let transfer = available.min(amount);

        *applied.get_mut(from).unwrap() -= transfer;
        *applied.get_mut(to).unwrap() += transfer;
    });

    values = applied.values().map(|v| *v).collect::<Vec<i64>>();
    values.sort();
    println!(
        "  │  ├─ Part 2: {}",
        values.iter().rev().take(3).sum::<i64>()
    );

    //println!("  │  └─ Part 3: {}", 3);
}
