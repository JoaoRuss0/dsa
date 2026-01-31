use std::collections::HashMap;

pub fn run() {
    println!("  ├─ Day 11 - Reactor");

    let path = "input/advent_of_code/Y2025/D11.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let racks = input
        .lines()
        .map(|l| {
            let split = l.split(": ").collect::<Vec<&str>>();
            let id = split[0].to_string();
            let connections = split[1]
                .split(" ")
                .map(str::to_string)
                .collect::<Vec<String>>();
            (id, connections)
        })
        .collect::<HashMap<String, Vec<String>>>();

    println!("  │  ├─ Part 1: {}", dfs("you", "out", &racks));
    //println!("  │  └─ Part 2: {}", );
}

fn dfs(start: &str, end: &str, graph: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();

    let mut stack = Vec::new();
    stack.push((start.to_string(), false));

    while let Some((curr, expanded)) = stack.pop() {
        if cache.contains_key(&curr) {
            continue;
        }

        if curr == end {
            cache.insert(curr, 1);
            continue;
        }

        match expanded {
            false => {
                stack.push((curr.clone(), true));
                if let Some(next) = graph.get(&curr) {
                    next.iter().for_each(|c| {
                        if !cache.contains_key(c.as_str()) {
                            stack.push((c.clone(), false));
                        }
                    })
                }
            }
            true => {
                let children_sum = graph
                    .get(&curr)
                    .map(|next| {
                        next.iter()
                            .map(|c| cache.get(c.as_str()).copied().unwrap_or(0))
                            .sum::<u64>()
                    })
                    .unwrap_or(0);
                cache.insert(curr, children_sum);
            }
        }
    }
    println!("{cache_hits}");

    *cache.get(start).unwrap()
}
