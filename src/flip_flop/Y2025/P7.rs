use crate::flip_flop::Y2025::P7::Direction::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub fn run() {
    println!("  ├─ Puzzle 6: Bird Spotters");

    let path = "input/flip_flop/Y2025/P7.txt";
    let input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  ├─ Part 1: {}",
        input
            .lines()
            .map(|l| {
                let split = l.split_once(" ").unwrap();
                let grid = Grid::new(split.0.parse().unwrap(), split.1.parse().unwrap());
                grid.find_all_shortest_paths()
            })
            .sum::<u16>()
    );
    //println!("  │  ├─ Part 2: {}",);
    //println!("  │  └─ Part 3: {}",);
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn move_in_all_directions(&self) -> Vec<Self> {
        Direction::ALL
            .iter()
            .map(|&d| self.move_in_direction(d))
            .filter_map(|p| p)
            .collect()
    }

    fn move_in_direction(&self, direction: Direction) -> Option<Self> {
        let v: Vector = direction.into();

        let n_x = self.x.checked_add(v.x);
        let n_y = self.y.checked_add(v.y);

        if n_x.is_none() || n_y.is_none() {
            return None;
        }

        Some(Self {
            x: n_x.unwrap(),
            y: n_y.unwrap(),
        })
    }
}

struct Grid {
    height: u8,
    width: u8,
    start: Point,
    end: Point,
}

impl Grid {
    fn new(width: u8, height: u8) -> Self {
        Self {
            start: Point { x: 0, y: 0 },
            end: Point {
                x: width as i16 - 1,
                y: height as i16 - 1,
            },
            height: 0,
            width: 0,
        }
    }

    fn find_all_shortest_paths(&self) -> u16 {
        #[derive(Copy, Clone, Eq)]
        struct Path {
            point: Point,
            length: u16,
        }

        impl Path {
            fn new(point: Point, length: u16) -> Self {
                Self { point, length }
            }
        }

        impl Hash for Path {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.point.hash(state);
            }
        }

        impl PartialEq for Path {
            fn eq(&self, other: &Self) -> bool {
                self.point == other.point
            }
        }

        struct Queue {
            queue: VecDeque<Path>,
            queued: HashMap<Path, u16>,
        }

        impl Queue {
            fn new() -> Self {
                Self {
                    queue: VecDeque::new(),
                    queued: HashMap::new(),
                }
            }

            fn push(&mut self, path: Path, amount: u16) {
                if self.queued.contains_key(&path) {
                    *self.queued.get_mut(&path).unwrap() += amount;
                    return;
                }
                self.queue.push_back(path);
                self.queued.insert(path, amount);
            }

            fn pop(&mut self) -> Option<(Path, u16)> {
                match self.queue.pop_front() {
                    Some(popped) => Some((popped, self.queued.remove(&popped).unwrap())),
                    None => return None,
                }
            }

            fn contains(&self, path: &Path) -> bool {
                self.queued.contains_key(path)
            }
        }

        let mut visited = HashSet::new();
        let mut length = None;
        let mut paths = 0;

        let mut queue = Queue::new();
        queue.push(Path::new(self.start, 0), 1);

        while let Some((path, amount)) = queue.pop() {
            if visited.contains(&path) {
                continue;
            }
            visited.insert(path);

            if path.point == self.end {
                match length {
                    Some(l) => {
                        if path.length == l {
                            paths += amount;
                        }
                    }
                    None => {
                        length = Some(path.length);
                        paths += amount;
                    }
                }
                continue;
            }

            for new in path.point.move_in_all_directions() {
                if self.is_out_of_bounds(new) {
                    continue;
                }
                queue.push(Path::new(new, path.length + 1), amount);
            }
        }
        paths
    }

    fn is_out_of_bounds(&self, point: Point) -> bool {
        point.x > self.end.x || point.x < 0 || point.y > self.end.y || point.y < 0
    }
}

struct Vector {
    x: i16,
    y: i16,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    const ALL: [Direction; 4] = [North, South, East, West];
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        match value {
            North => Vector { x: 1, y: 0 },
            West => Vector { x: 0, y: 1 },
            South => Vector { x: -1, y: 0 },
            East => Vector { x: 0, y: -1 },
        }
    }
}
