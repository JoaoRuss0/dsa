use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("  ├─ Quest 5 - Pseudo-Random Clap Dance");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/Q5/P1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();
    println!(
        "  │  ├─ Part 1: {}",
        part1(&mut get_dance_floor(&input), 10)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/Q5/P2.txt";
    input = std::fs::read_to_string(path).unwrap();
    println!(
        "  │  ├─ Part 2: {}",
        part2(&mut get_dance_floor(&input), 2024)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/Q5/P3.txt";
    input = std::fs::read_to_string(path).unwrap();
    println!("  │  └─ Part 3: {}", part3(&mut get_dance_floor(&input)));
}

fn get_dance_floor(input: &str) -> Vec<Vec<u32>> {
    let mut dance_floor: Vec<Vec<u32>> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        for (col_idx, n) in line.split_whitespace().enumerate() {
            match dance_floor.get_mut(col_idx) {
                Some(_) => {}
                None => dance_floor.push(Vec::new()),
            }

            dance_floor
                .get_mut(col_idx)
                .unwrap()
                .push(n.parse::<u32>().unwrap());
        }
    }

    dance_floor
}

fn part1(dance_floor: &mut [Vec<u32>], iterations: usize) -> u64 {
    let mut n = 0;

    for i in 0..iterations {
        n = dance(dance_floor, i % dance_floor.len());
    }

    n
}

fn part2(dance_floor: &mut [Vec<u32>], shouts: usize) -> u64 {
    let mut shouts_per_number: HashMap<u64, usize> = HashMap::new();
    let mut max = 0;
    let mut n = 0;

    let mut i = 0;
    while max < shouts {
        n = dance(dance_floor, i % dance_floor.len());

        let entry = shouts_per_number.entry(n).or_default();
        *entry += 1;
        max = max.max(*entry);

        i += 1;
    }

    n * i as u64
}

fn part3(dance_floor: &mut Vec<Vec<u32>>) -> u64 {
    let mut seen = HashSet::new();
    let mut max = u64::MIN;
    let len = dance_floor.len();

    let mut i = 0;
    loop {
        if seen.insert(dance_floor.clone()) {
            max = max.max(dance(dance_floor, i % len));
            i += 1;
            continue;
        }
        break;
    }

    max
}

fn dance(dance_floor: &mut [Vec<u32>], column: usize) -> u64 {
    let len = dance_floor.len();
    let move_to = (column + 1) % len;
    let clapper = dance_floor[column].remove(0);
    let index = get_index_on_new_column(dance_floor, move_to, clapper);
    dance_floor[move_to].insert(index, clapper);

    dance_floor
        .iter()
        .map(|column| column[0].to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn get_index_on_new_column(dance_floor: &[Vec<u32>], column: usize, clapper: u32) -> usize {
    let len = dance_floor[column].len();
    let laps = clapper / len as u32;
    let side = laps % 2;
    let row = clapper % len as u32;

    match (side, row) {
        (0, 0) => 1,
        (1, 0) => len - 1,
        (0, _) => row as usize - 1,
        (1, _) => len - row as usize + 1,
        _ => panic!("Invalid side"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut dance_floor = vec![vec![2, 3, 4], vec![5, 6, 7]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![3, 4], vec![5, 2, 6, 7]], dance_floor);
    }

    #[test]
    fn test2() {
        let mut dance_floor = vec![vec![3, 9, 4], vec![5, 6, 7]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 4], vec![5, 6, 3, 7]], dance_floor);
    }

    #[test]
    fn test3() {
        let mut dance_floor = vec![vec![4, 9, 3], vec![5, 6, 7]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 3], vec![5, 6, 7, 4]], dance_floor);
    }

    #[test]
    fn test4() {
        let mut dance_floor = vec![vec![5, 9, 3], vec![4, 6, 7]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 3], vec![4, 6, 5, 7]], dance_floor);
    }

    #[test]
    fn test5() {
        let mut dance_floor = vec![vec![6, 9, 3], vec![4, 5, 7]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 3], vec![4, 6, 5, 7]], dance_floor);
    }

    #[test]
    fn test6() {
        let mut dance_floor = vec![vec![24, 9, 3], vec![4, 5, 8]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 3], vec![4, 24, 5, 8]], dance_floor);
    }

    #[test]
    fn test7() {
        let mut dance_floor = vec![vec![1, 9, 3], vec![4, 5, 8]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 3], vec![1, 4, 5, 8]], dance_floor);
    }

    #[test]
    fn test8() {
        let mut dance_floor = vec![vec![7, 9, 3], vec![4, 5, 7]];
        dance(&mut dance_floor, 0);
        assert_eq!(vec![vec![9, 3], vec![7, 4, 5, 7]], dance_floor);
    }
}
