use std::collections::{HashMap, VecDeque};

pub fn run() {
    println!("  ├─ Day 7 - Laboratories");

    let path = "input/advent_of_code/Y2025/D7.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let manifold = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = Point { x: 0, y: 0 };
    'outer: for i in 0..manifold.len() {
        for j in 0..manifold[i].len() {
            if manifold[i][j] == 'S' {
                start = Point {
                    x: j as i32,
                    y: i as i32,
                };
                break 'outer;
            }
        }
    }

    let mut splits: u64 = 0;
    let mut timelines: u64 = 0;

    let mut to_visit = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    to_visit.insert(start, 1);

    while let Some(curr) = queue.pop_front() {
        let quantity = to_visit.remove(&curr).unwrap();

        if curr.y == manifold.len() as i32 - 1 {
            timelines += quantity;
            continue;
        }

        let next_pos = curr.down();
        match manifold[next_pos.y as usize][next_pos.x as usize] == '^' {
            true => {
                splits += 1;

                let (next_right, next_left) = curr.split();
                if !to_visit.contains_key(&next_right) {
                    queue.push_back(next_right);
                }
                if !to_visit.contains_key(&next_left) {
                    queue.push_back(next_left);
                }
                *to_visit.entry(next_right).or_insert(0) += quantity;
                *to_visit.entry(next_left).or_insert(0) += quantity;
            }
            false => {
                if !to_visit.contains_key(&next_pos) {
                    queue.push_back(next_pos);
                }
                *to_visit.entry(next_pos).or_insert(0) += quantity;
            }
        }
    }

    println!("  │  ├─ Part 1: {}", splits);
    println!("  │  └─ Part 2: {}", timelines);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn down(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }
    fn split(&self) -> (Self, Self) {
        (
            Self {
                y: self.y + 1,
                x: self.x - 1,
            },
            Self {
                y: self.y + 1,
                x: self.x + 1,
            },
        )
    }
}
