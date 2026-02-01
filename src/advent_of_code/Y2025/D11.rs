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

    println!("  │  ├─ Part 1: {}", dfs("svr", "out", &racks));
    //println!("  │  └─ Part 2: {}", );
}

fn dfs(start: &str, end: &str, graph: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();

    let mut stack = Vec::new();
    stack.push(((start.to_string(), start == "dac", start == "fft"), false));

    while let Some(((curr, dac, fft), expanded)) = stack.pop() {
        if cache.contains_key(&(curr.clone(), dac, fft)) {
            continue;
        }

        if curr == end {
            cache.insert((curr, dac, fft), if dac && fft { 1 } else { 0 });
            continue;
        }

        match expanded {
            false => {
                stack.push(((curr.clone(), dac, fft), true));
                if let Some(next) = graph.get(&curr) {
                    next.iter().for_each(|c| {
                        let key = (c.clone(), dac || c == "dac", fft || c == "fft");
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
                                let key = (c.clone(), dac || c == "dac", fft || c == "fft");
                                cache.get(&key).copied().unwrap_or(0)
                            })
                            .sum::<u64>()
                    })
                    .unwrap_or(0);
                cache.insert((curr, dac, fft), children_sum);
            }
        }
    }

    *cache.get(&(start.to_string(), false, false)).unwrap()
}
