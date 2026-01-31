use std::collections::{HashMap, VecDeque};

pub fn run() {
    println!("  ├─ Day 11 - Reactor");

    let path = "input/advent_of_code/Y2025/D11.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let racks = input
        .lines()
        .map(|l| {
            let split = l.split(": ").collect::<Vec<&str>>();
            let id = split[0];
            let connections = split[1].split(" ").collect::<Vec<&str>>();
            (id, connections)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    println!("  │  ├─ Part 1: {}", dfs("you", "out", &racks));
    //println!("  │  └─ Part 2: {}", );
}

fn dfs(start: &str, end: &str, graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut paths = 0;

    while let Some(next) = queue.pop_front() {
        match next == end {
            true => paths += 1,
            false => graph
                .get(&next)
                .unwrap()
                .iter()
                .for_each(|&id| queue.push_back(id)),
        }
    }

    paths
}
