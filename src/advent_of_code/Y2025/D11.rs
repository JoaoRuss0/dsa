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
    println!(
        "  │  └─ Part 2: {}",
        dfs_visit("svr", "out", &racks, &["dac", "fft"])
    );
}

fn dfs(start: &str, end: &str, graph: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();

    let mut stack = Vec::new();
    stack.push((start.to_string(), false));

    while let Some((curr, expanded)) = stack.pop() {
        if cache.contains_key(&curr.clone()) {
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
                            .map(|c| cache.get(c).copied().unwrap_or(0))
                            .sum::<u64>()
                    })
                    .unwrap_or(0);
                cache.insert(curr, children_sum);
            }
        }
    }

    *cache.get(&start.to_string()).unwrap()
}

fn dfs_visit(
    start: &str,
    end: &str,
    graph: &HashMap<String, Vec<String>>,
    to_visit: &[&str],
) -> u64 {
    let fully_visited = (1_u64 << to_visit.len()) - 1;
    let mut bit_assignment = HashMap::new();
    to_visit.iter().enumerate().for_each(|(i, &v)| {
        bit_assignment.insert(v, 1_u64 << i);
    });
    let get_bit = |v: &str| bit_assignment.get(v).copied().unwrap_or(0);

    let mut cache = HashMap::new();

    let mut stack = Vec::new();
    stack.push(((start.to_string(), get_bit(start)), false));

    while let Some(((curr, visited), expanded)) = stack.pop() {
        if cache.contains_key(&(curr.clone(), visited)) {
            continue;
        }

        if curr == end {
            cache.insert(
                (curr, visited),
                if visited == fully_visited { 1 } else { 0 },
            );
            continue;
        }

        match expanded {
            false => {
                stack.push(((curr.clone(), visited), true));
                if let Some(next) = graph.get(&curr) {
                    next.iter().for_each(|c| {
                        let key = (c.clone(), visited | get_bit(&c.as_str()));
                        if !cache.contains_key(&key) {
                            stack.push((key, false));
                        }
                    })
                }
            }
            true => {
                let children_sum = graph
                    .get(&curr)
                    .map(|next| {
                        next.iter()
                            .map(|c| {
                                let key = (c.clone(), visited | get_bit(&c.as_str()));
                                cache.get(&key).copied().unwrap_or(0)
                            })
                            .sum::<u64>()
                    })
                    .unwrap_or(0);
                cache.insert((curr, visited), children_sum);
            }
        }
    }

    *cache.get(&(start.to_string(), get_bit(start))).unwrap()
}
