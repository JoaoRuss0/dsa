use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

pub fn run() {
    println!("  ├─ Day 6 - Guard Gallivant");

    let path = "input/advent_of_code/Y2024/D6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (mut grid, start) = parse(input);
    let start = Pose::new(start, Direction::North);
    let (mut poses, _) = walk(start, &mut grid);

    println!(
        "  │  ├─ Part 1: {}",
        poses.iter().map(|p| p.p).collect::<HashSet<Point>>().len()
    );
    println!("  │  └─ Part 2: {}", loops(poses, &mut grid));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn direct(self, direction: &Direction) -> Point {
        let (x, y) = direction.value();
        Self::new(self.x + x, self.y + y)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Pose {
    p: Point,
    d: Direction,
}

impl Pose {
    fn new(p: Point, d: Direction) -> Self {
        Self { p, d }
    }

    fn direct(self) -> Self {
        Self::new(self.p.direct(&self.d), self.d)
    }

    fn rotate_clockwise(&mut self) {
        self.d = self.d.rotate_clockwise();
    }
}

fn loops(poses: BTreeSet<Pose>, grid: &mut Vec<Vec<char>>) -> usize {
    let mut loops = 0;

    for p in poses {
        let o = p.direct();

        if o.p.x < 0
            || o.p.y < 0
            || o.p.x >= grid.len() as i32
            || o.p.y >= grid[0].len() as i32
            || grid[o.p.x as usize][o.p.y as usize] == '#'
        {
            continue;
        }

        grid[o.p.x as usize][o.p.y as usize] = '#';

        if walk(p, grid).1 {
            loops += 1;
        }

        grid[o.p.x as usize][o.p.y as usize] = '.';
    }

    loops
}

fn walk(p: Pose, grid: &Vec<Vec<char>>) -> (BTreeSet<Pose>, bool) {
    let mut poses = BTreeSet::new();
    let mut looped = false;

    let mut curr = p;

    loop {
        if poses.contains(&curr) {
            looped = true;
            break;
        }

        poses.insert(curr);

        let next = curr.direct();
        if next.p.x < 0
            || next.p.y < 0
            || next.p.x >= grid.len() as i32
            || next.p.y >= grid[0].len() as i32
        {
            break;
        }

        if grid[next.p.x as usize][next.p.y as usize] == '#' {
            curr.rotate_clockwise();
            continue;
        }

        curr = next;
    }

    (poses, looped)
}

fn parse(input: String) -> (Vec<Vec<char>>, Point) {
    let mut grid = Vec::new();
    let mut start = None;

    for (x, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (y, c) in line.chars().enumerate() {
            if c == '^' {
                start = Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }

            grid.last_mut().unwrap().push(c);
        }
    }

    (grid, start.unwrap())
}
