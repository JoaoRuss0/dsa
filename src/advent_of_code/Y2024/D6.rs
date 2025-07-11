use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Day 6 - Guard Gallivant");

    let path = "input/advent_of_code/Y2024/D6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let (mut grid, start) = parse(input);

    println!(
        "  │  ├─ Part 1: {}",
        run_loop(start, Direction::North, &mut grid)
    );
    //println!("  │  └─ Part 2: {}", );
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    fn next_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn run_loop(mut point: Point, mut direction: Direction, grid: &mut Vec<Vec<char>>) -> usize {
    let mut positions = HashSet::new();

    loop {
        grid[point.x as usize][point.y as usize] = 'H';
        positions.insert(point);

        let next = point.direct(&direction);
        if next.x < 0 || next.y < 0 || next.x >= grid.len() as i32 || next.y >= grid[0].len() as i32
        {
            break;
        }

        if grid[next.x as usize][next.y as usize] == '#' {
            direction = direction.next_clockwise();
            continue;
        }

        point = next;
    }

    positions.len()
}

fn parse(input: String) -> (Vec<Vec<char>>, Point) {
    let mut grid = Vec::new();
    let mut start = None;

    for (x, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for (y, char) in line.chars().enumerate() {
            let mut char = char;
            if char == '^' {
                start = Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
                char = '#';
            }

            grid.last_mut().unwrap().push(char);
        }
    }

    (grid, start.unwrap())
}
