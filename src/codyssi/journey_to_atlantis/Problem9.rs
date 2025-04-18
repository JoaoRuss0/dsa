use std::collections::{HashMap, VecDeque};

pub fn run() {
    println!("  ├─ Problem 9 - Windy Bargain");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem9.txt").unwrap();

    let mut sections = input.split("\n\n").collect::<VecDeque<&str>>();

    let mut balances: HashMap<String, i64> = HashMap::new();
    sections
        .pop_front()
        .unwrap()
        .lines()
        .map(|b| b.split_once(" HAS ").unwrap())
        .for_each(|(name, balance)| {
            balances.insert(name.to_string(), balance.parse::<i64>().unwrap());
        });

    let transactions: Vec<(String, String, i64)> = sections
        .pop_back()
        .unwrap()
        .lines()
        .map(|b| b.split_whitespace().collect::<Vec<&str>>())
        .map(|s| {
            (
                s[1].to_string(),
                s[3].to_string(),
                s[5].parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut applied = balances.clone();
    transactions
        .iter()
        .for_each(|(from, to, amount)| transfer(from, to, *amount, &mut applied));

    let mut values = applied.values().copied().collect::<Vec<i64>>();
    values.sort();
    println!(
        "  │  ├─ Part 1: {:?}",
        values.iter().rev().take(3).sum::<i64>()
    );

    applied = balances.clone();
    transactions.iter().for_each(|(from, to, amount)| {
        let available = *applied.get(from).unwrap();
        let transfer = available.min(*amount);

        *applied.get_mut(from).unwrap() -= transfer;
        *applied.get_mut(to).unwrap() += transfer;
    });

    values = applied.values().copied().collect::<Vec<i64>>();
    values.sort();
    println!(
        "  │  ├─ Part 2: {}",
        values.iter().rev().take(3).sum::<i64>()
    );

    applied = balances.clone();
    let mut debts: HashMap<&String, VecDeque<(&String, i64)>> = HashMap::new();
    transactions.iter().for_each(|(from, to, amount)| {
        let mut to_transfer = *amount;
        let available = *applied.get(from).unwrap();

        if available < *amount {
            to_transfer = available;
            debts
                .entry(from)
                .or_default()
                .push_back((to, amount - available));
        }

        transfer(from, to, to_transfer, &mut applied);
        pay_debt(to, &mut debts, &mut applied);
    });

    values = applied.values().copied().collect::<Vec<i64>>();
    values.sort();
    println!(
        "  │  └─ Part 3: {}",
        values.iter().rev().take(3).sum::<i64>()
    );
}

pub fn transfer(from: &String, to: &String, amount: i64, balances: &mut HashMap<String, i64>) {
    *balances.get_mut(to).unwrap() += amount;
    *balances.get_mut(from).unwrap() -= amount;
}

pub fn pay_debt(
    name: &String,
    debts: &mut HashMap<&String, VecDeque<(&String, i64)>>,
    balances: &mut HashMap<String, i64>,
) {
    while let Some(balance) = balances.get_mut(name) {
        if *balance == 0 {
            return;
        }

        if debts.get_mut(name).is_none() {
            return;
        }

        let debts_for_name = debts.get_mut(name).unwrap();
        if debts_for_name.is_empty() {
            return;
        }

        let (to, amount) = debts_for_name.pop_front().unwrap();

        let mut to_transfer = amount;
        let available = *balances.get(name).unwrap();
        if available < amount {
            debts
                .get_mut(name)
                .unwrap()
                .push_front((to, amount - available));
            to_transfer = available;
        }

        transfer(name, to, to_transfer, balances);
        pay_debt(to, debts, balances);
    }
}
