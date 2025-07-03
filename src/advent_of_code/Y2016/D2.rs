pub fn run() {
    println!("  ├─ Day 2 - Bathroom Security");

    let path = "input/advent_of_code/Y2016/D2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let instructions = input
        .lines()
        .map(|l| {
            l.chars()
                .map(Direction::from_char)
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Vec<Direction>>>();

    let mut p = Pad::new();
    let values = instructions
        .into_iter()
        .map(|i| p.apply(i))
        .collect::<Vec<usize>>();

    println!("  │  ├─ Part 1: {values:?}");
    //println!("  │  └─ Part 2: {}", );
}

struct Pad {
    grid: Vec<Vec<usize>>,
    pos: Position,
}

impl Pad {
    fn new() -> Pad {
        Pad {
            grid: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            pos: Position { x: 1, y: 1 },
        }
    }

    fn apply(&mut self, directions: Vec<Direction>) -> usize {
        let is_outside_grid = |x: i32, y: i32, grid: &Vec<Vec<usize>>| {
            x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32
        };

        for d in directions {
            let (nx, ny) = (
                self.pos.x as i32 + d.offset().0,
                self.pos.y as i32 + d.offset().1,
            );

            if is_outside_grid(nx, ny, &self.grid) {
                continue;
            }

            self.pos.x = nx as usize;
            self.pos.y = ny as usize;
        }

        self.grid[self.pos.x][self.pos.y]
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

struct Position {
    x: usize,
    y: usize,
}
