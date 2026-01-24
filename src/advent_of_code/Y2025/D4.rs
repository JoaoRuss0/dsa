use crate::advent_of_code::Y2025::D4::Direction::*;

pub fn run() {
    println!("  ├─ Day 4 - Printing Department");

    let path = "input/advent_of_code/Y2025/D4.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let grid = Grid::from(input);

    println!("  │  ├─ Part 1: {}", grid.count_accessible_rolls());
    //println!("  │  └─ Part 2: {}", );
}

struct Grid {
    inner: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn from(lines: String) -> Self {
        Self {
            inner: lines.lines().map(|l| l.chars().collect()).collect(),
            height: lines.lines().count(),
            width: lines.lines().next().unwrap().len(),
        }
    }

    fn count_accessible_rolls(&self) -> usize {
        let mut accessible = 0;

        let count_adjacent_rolls = |from: &Point| -> usize {
            let mut rolls = 0;

            for &dir in Direction::ALL.iter() {
                let vector = dir.into();
                let to_check = from.apply(&vector);
                if self.is_out_of_bounds(&to_check)
                    || self.inner[to_check.x as usize][to_check.y as usize] != '@'
                {
                    continue;
                }

                rolls += 1;
            }

            rolls
        };

        for (x, row) in self.inner.iter().enumerate() {
            for (y, &value) in row.iter().enumerate() {
                if value != '@'
                    || count_adjacent_rolls(&Point {
                        x: x as i32,
                        y: y as i32,
                    }) >= 4
                {
                    continue;
                }
                accessible += 1;
            }
        }

        accessible
    }

    fn is_out_of_bounds(&self, point: &Point) -> bool {
        point.x < 0 || point.x >= self.height as i32 || point.y < 0 || point.y >= self.width as i32
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply(&self, vector: &Vector) -> Self {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    const ALL: [Direction; 8] = [
        North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
    ];
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        match value {
            North => Vector { x: 0, y: 1 },
            NorthEast => Vector { x: 1, y: 1 },
            East => Vector { x: 1, y: 0 },
            SouthEast => Vector { x: 1, y: -1 },
            South => Vector { x: 0, y: -1 },
            SouthWest => Vector { x: -1, y: -1 },
            West => Vector { x: -1, y: 0 },
            NorthWest => Vector { x: -1, y: 1 },
        }
    }
}
