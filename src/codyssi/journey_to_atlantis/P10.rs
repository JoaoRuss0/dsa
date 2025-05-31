use std::collections::{BinaryHeap, HashSet};

pub fn run() {
    println!("  ├─ Problem 10 - Cyclops Chaos");

    let path = "input/codyssi/journey_to_atlantis/P10.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut grid: Vec<Vec<i64>> = Vec::new();
    input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .for_each(|row| grid.push(row));

    let mut safest = i64::MAX;
    for i in 0..grid.len() {
        let mut current = 0;
        for j in 0..grid[0].len() {
            current += grid[i][j];
        }
        safest = safest.min(current);
    }

    for i in 0..grid[0].len() {
        let mut current = 0;
        for j in 0..grid.len() {
            current += grid[j][i];
        }
        safest = safest.min(current);
    }

    println!("  │  ├─ Part 1: {}", safest);
    println!(
        "  │  ├─ Part 2: {}",
        find_safest_path(&grid, (0, 0), (14, 14))
    );
    println!(
        "  │  └─ Part 3: {}",
        find_safest_path(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1))
    );
}

fn find_safest_path(grid: &[Vec<i64>], start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut queue: BinaryHeap<Path> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut goal = 0;

    queue.push(Path {
        danger: grid[start.0][start.1],
        position: (start.0, start.1),
    });

    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        let (x, y) = p.position;
        if x == end.0 && y == end.1 {
            goal = p.danger;
            break;
        }

        if !visited.insert(p.position) {
            continue;
        }

        if p.position.0 < end.0 {
            queue.push(Path {
                danger: p.danger + grid[x + 1][y],
                position: (x + 1, y),
            });
        }

        if p.position.1 < end.1 {
            queue.push(Path {
                danger: p.danger + grid[x][y + 1],
                position: (x, y + 1),
            });
        }
    }

    goal
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Path {
    danger: i64,
    position: (usize, usize),
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.danger.cmp(&self.danger)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
