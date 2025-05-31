use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    println!("  └─ Problem 4 - Traversing the Country");

    let path = "input/codyssi/summer_at_the_lab/P4.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut locations: HashMap<&str, HashSet<&str>> = HashMap::new();
    input
        .lines()
        .map(|line| line.split_once(" <-> ").unwrap())
        .for_each(|(l1, l2)| {
            locations.entry(l1).or_default().insert(l2);
            locations.entry(l2).or_default().insert(l1);
        });

    println!("     ├─ Part 1: {}", locations.len());

    let mut visited = HashSet::new();
    let mut queue: VecDeque<(u8, &str)> = VecDeque::new();
    queue.push_back((0, "STT"));

    while let Some((cost, location)) = queue.pop_front() {
        if cost > 3 {
            break;
        };

        visited.insert(location);
        locations
            .get(location)
            .unwrap()
            .iter()
            .filter(|&l| !visited.contains(l))
            .for_each(|&l| queue.push_back((cost + 1, l)));
    }

    println!("     ├─ Part 2: {}", visited.len());

    let mut seen = HashSet::new();
    let mut queue: VecDeque<(u8, &str)> = VecDeque::new();
    queue.push_back((0, "STT"));
    seen.insert("STT");

    let mut time: u64 = 0;
    while let Some((cost, location)) = queue.pop_front() {
        time += cost as u64;
        for l in locations.get(location).unwrap() {
            if seen.contains(l) {
                continue;
            }
            queue.push_back((cost + 1, *l));
            seen.insert(*l);
        }
    }

    println!("     └─ Part 3: {}", time);
}
