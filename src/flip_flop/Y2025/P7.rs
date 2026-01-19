use crate::flip_flop::Y2025::P7::Direction::*;
use crate::flip_flop::Y2025::P7::Direction3D::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub fn run() {
    println!("  ├─ Puzzle 7: Hyper Grids");

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

    println!(
        "  │  ├─ Part 2: {}",
        input
            .lines()
            .map(|l| {
                let split = l.split_once(" ").unwrap();
                let grid = Grid3D::new(
                    split.0.parse().unwrap(),
                    split.1.parse().unwrap(),
                    split.0.parse().unwrap(),
                );
                grid.find_all_shortest_paths()
            })
            .sum::<u64>()
    );
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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point3D {
    x: i16,
    y: i16,
    z: i16,
}

impl Point3D {
    fn move_in_all_directions(&self) -> Vec<Self> {
        Direction3D::ALL
            .iter()
            .map(|&d| self.move_in_direction(d))
            .filter_map(|p| p)
            .collect()
    }

    fn move_in_direction(&self, direction: Direction3D) -> Option<Self> {
        let v: Vector3D = direction.into();

        let n_x = self.x.checked_add(v.x);
        let n_y = self.y.checked_add(v.y);
        let n_z = self.z.checked_add(v.z);

        if n_x.is_none() || n_y.is_none() || n_z.is_none() {
            return None;
        }

        Some(Self {
            x: n_x.unwrap(),
            y: n_y.unwrap(),
            z: n_z.unwrap(),
        })
    }
}

#[derive(Copy, Clone, Eq)]
struct Path3D {
    point: Point3D,
    length: u64,
}

impl Path3D {
    fn new(point: Point3D, length: u64) -> Self {
        Self { point, length }
    }
}

impl Hash for Path3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

impl PartialEq for Path3D {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

struct Queue {
    queue: VecDeque<Path3D>,
    queued: HashMap<Path3D, u64>,
}

impl Queue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            queued: HashMap::new(),
        }
    }

    fn push(&mut self, path: Path3D, amount: u64) {
        if self.queued.contains_key(&path) {
            *self.queued.get_mut(&path).unwrap() += amount;
            return;
        }
        self.queue.push_back(path);
        self.queued.insert(path, amount);
    }

    fn pop(&mut self) -> Option<(Path3D, u64)> {
        match self.queue.pop_front() {
            Some(popped) => Some((popped, self.queued.remove(&popped).unwrap())),
            None => None,
        }
    }

    fn contains(&self, path: &Path3D) -> bool {
        self.queued.contains_key(path)
    }
}

struct Grid3D {
    x_len: u8,
    y_len: u8,
    z_len: u8,
    start: Point3D,
    end: Point3D,
}

impl Grid3D {
    fn new(x_len: u8, y_len: u8, z_len: u8) -> Self {
        Self {
            start: Point3D { x: 0, y: 0, z: 0 },
            end: Point3D {
                x: x_len as i16 - 1,
                y: y_len as i16 - 1,
                z: z_len as i16 - 1,
            },
            x_len,
            y_len,
            z_len,
        }
    }

    fn find_all_shortest_paths(&self) -> u64 {
        let mut visited = HashSet::new();
        let mut length = None;
        let mut paths = 0;

        let mut queue = Queue::new();
        queue.push(Path3D::new(self.start, 0), 1);

        while let Some((path, amount)) = queue.pop() {
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

                match length {
                    Some(l) => {
                        if path.length + 1 > l {
                            continue;
                        }
                    }
                    None => queue.push(Path3D::new(new, path.length + 1), amount),
                }
            }
        }
        paths
    }

    fn is_out_of_bounds(&self, point: Point3D) -> bool {
        point.x > self.end.x
            || point.x < 0
            || point.y > self.end.y
            || point.y < 0
            || point.z > self.end.z
            || point.z < 0
    }
}

struct Vector3D {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Copy, Clone)]
enum Direction3D {
    Up,
    Right,
    Down,
    Left,
    Front,
    Back,
}

impl Direction3D {
    const ALL: [Direction3D; 6] = [Up, Right, Down, Left, Front, Back];
}

impl From<Direction3D> for Vector3D {
    fn from(value: Direction3D) -> Self {
        match value {
            Up => Vector3D { x: 0, y: 1, z: 0 },
            Right => Vector3D { x: 1, y: 0, z: 0 },
            Down => Vector3D { x: 0, y: -1, z: 0 },
            Left => Vector3D { x: -1, y: 0, z: 0 },
            Front => Vector3D { x: 0, y: 0, z: -1 },
            Back => Vector3D { x: 0, y: 0, z: 1 },
        }
    }
}
