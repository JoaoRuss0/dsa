use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Problem 3 - Supplies in Surplus");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem3.txt").unwrap();

    let get_range = |r : &str| -> HashSet<i64> {
        let (start, end) : (&str, &str) = r.split_once('-').unwrap();
        (start.parse::<i64>().unwrap()..=end.parse::<i64>().unwrap()).collect::<HashSet<i64>>()
    };

    let piles = input.lines()
        .map(|p| p.split_once(" ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let count = piles.clone()
        .iter()
        .flat_map(|(r1, r2)| [r1, r2])
        .map(|r| get_range(r).len() as i64)
        .sum::<i64>();

    println!("  │  ├─ Part 1: {}", count);

    let pile_count = piles.clone()
        .iter()
        .map(|(r1, r2)| {
            let mut set = get_range(r1);
            set.extend(get_range(r2).iter());
            set.len() as i64
        })
        .sum::<i64>();

    println!("  │  ├─ Part 2: {}", pile_count);

    let adjacent_piles_count = piles.clone()
        .windows(2)
        .map(|pair| {
            let mut set = get_range(pair[0].0);
            set.extend(get_range(pair[0].1).iter());
            set.extend(get_range(pair[1].0).iter());
            set.extend(get_range(pair[1].1).iter());
            set.len() as i64
        })
        .max()
        .unwrap();

    println!("  │  └─ Part 3: {}", adjacent_piles_count);
}
