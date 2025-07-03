use crate::advent_of_code::Y2016::D1::Direction::*;
use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Day 1 - No Time for a Taxicab");

    let path = "input/advent_of_code/Y2016/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let instructions = get_instructions(&input);

    let mut p = Pose {
        pos: Position::new(),
        dir: Up,
    };

    let mut twice: Option<Position> = None;
    let mut seen = HashSet::new();
    seen.insert(p.pos);

    for i in instructions {
        let positions = p.apply(i);
        positions.into_iter().for_each(|p| {
            if twice.is_none() {
                match seen.get(&p) {
                    None => {
                        seen.insert(p);
                    }
                    Some(_) => twice = Some(p),
                }
            }
        });
    }

    println!("  │  ├─ Part 1: {}", p.pos.x.abs() + p.pos.y.abs());
    let unwrapped = twice.unwrap();
    println!("  │  └─ Part 2: {}", unwrapped.x.abs() + unwrapped.y.abs());
}

#[derive(Copy, Clone)]
enum Rotation {
    Left,
    Right,
}

impl Rotation {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _ => panic!("Not a valid rotation"),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(self) -> (i32, i32) {
        match self {
            Up => (1, 0),
            Down => (-1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
    fn rotate(self, r: Rotation) -> Direction {
        match r {
            Rotation::Left => match self {
                Up => Left,
                Right => Up,
                Down => Right,
                Left => Down,
            },
            Rotation::Right => match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
        }
    }
}

struct Instruction {
    rotation: Rotation,
    times: usize,
}

#[derive(Copy, Clone)]
struct Pose {
    pos: Position,
    dir: Direction,
}

impl Pose {
    fn apply(&mut self, i: Instruction) -> Vec<Position> {
        let d = self.dir.rotate(i.rotation);
        let offset = d.offset();

        self.dir = d;
        self.pos.add(offset, i.times)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn add(&mut self, offset: (i32, i32), times: usize) -> Vec<Position> {
        let (x, y) = offset;

        let mut positions = vec![];

        for _ in 0..times {
            self.x += x;
            self.y += y;
            positions.push(*self);
        }

        positions
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| {
            let chars = s.chars().collect::<Vec<char>>();
            let num = chars[1..]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            Instruction {
                rotation: Rotation::from_char(chars[0]),
                times: num,
            }
        })
        .collect::<Vec<Instruction>>()
}
