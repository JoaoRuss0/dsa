use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn run() {
    println!("  ├─ Problem 13 - Laestrygonian Guards");

    let path = "input/codyssi/journey_to_atlantis/P13.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut nodes: HashMap<String, HashMap<String, u32>> = HashMap::new();
    input.lines().for_each(|line| {
        let (path, value) = line.split_once(" | ").unwrap();
        let (start, end) = path.split_once(" -> ").unwrap();
        nodes
            .entry(start.to_string())
            .or_default()
            .insert(end.to_string(), value.parse().unwrap());
    });

    let mut costs = get_paths_for_key("STT".to_string(), &nodes, Some(1));
    costs.sort_by(|prev, next| next.1.cmp(&prev.1));
    println!(
        "  │  ├─ Part 1: {}",
        costs.iter().take(3).map(|p| p.1).product::<u32>()
    );

    let mut path_costs = get_paths_for_key("STT".to_string(), &nodes, None);
    path_costs.sort_by(|prev, next| next.1.cmp(&prev.1));
    println!(
        "  │  ├─ Part 2: {}",
        path_costs.iter().take(3).map(|p| p.1).product::<u32>()
    );

    let mut priciest_cycle = 0;
    for (key, _) in nodes.iter() {
        let ends_in_start = nodes
            .iter()
            .filter(|n| n.1.contains_key(key))
            .map(|n| n.0.clone())
            .collect::<Vec<String>>();
        if ends_in_start.is_empty() {
            continue;
        }

        let paths_for_key = get_paths_for_key(key.to_string(), &nodes, None);
        if paths_for_key.is_empty() {
            continue;
        }

        if let Some((node, cost)) = paths_for_key.iter().find(|&p| ends_in_start.contains(&p.0)) {
            let cost_to_cycle = nodes.get(node).unwrap().get(key).unwrap();
            priciest_cycle = priciest_cycle.max(*cost + cost_to_cycle);
        }
    }

    println!("  │  └─ Part 3: {}", priciest_cycle);
}

fn get_paths_for_key(
    start: String,
    nodes: &HashMap<String, HashMap<String, u32>>,
    o_cost: Option<u32>,
) -> Vec<(String, u32)> {
    nodes
        .keys()
        .filter_map(|key| find_lowest_path_cost(start.clone(), key.to_string(), nodes, o_cost))
        .collect::<Vec<(String, u32)>>()
}

fn find_lowest_path_cost(
    start: String,
    end: String,
    nodes: &HashMap<String, HashMap<String, u32>>,
    o_cost: Option<u32>,
) -> Option<(String, u32)> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: BinaryHeap<PathCost> = BinaryHeap::new();

    queue.push(PathCost {
        c_node: start,
        cost: 0,
    });
    while let Some(path) = queue.pop() {
        if path.c_node == end {
            return Some((path.c_node, path.cost));
        }

        if visited.contains(&path.c_node) {
            continue;
        }
        visited.insert(path.c_node.clone());

        if let Some(children) = nodes.get(&path.c_node) {
            children.iter().for_each(|(child, cost)| {
                queue.push(PathCost {
                    c_node: child.to_string(),
                    cost: if let Some(c) = o_cost {
                        path.cost + c
                    } else {
                        path.cost + cost
                    },
                });
            })
        }
    }
    None
}

#[derive(Eq, PartialEq)]
struct PathCost {
    c_node: String,
    cost: u32,
}

impl Ord for PathCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
