pub fn run() {
    println!("  ├─ Day 4 - Ceres Search");

    let path = "input/advent_of_code/Y2024/D4.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let grid = Grid::parse(&input);

    println!("  │  ├─ Part 1: {}", grid.matches("XMAS"));
    //println!("  │  └─ Part 2: {}", );
}

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
    fn values(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, 1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        }
    }

    fn entries() -> [Direction; 8] {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }
}

struct Grid {
    inner: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();

        Grid { inner: grid }
    }

    fn matches(&self, term: &str) -> usize {
        let mut count = 0;

        for i in 0..self.inner.len() {
            for (j, &c) in self.inner.get(i).unwrap().into_iter().enumerate() {
                if c != term.chars().next().unwrap() {
                    continue;
                }

                for d in Direction::entries() {
                    if self.search_pos_in_direction(i, j, d, term) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn search_pos_in_direction(
        &self,
        row: usize,
        column: usize,
        direction: Direction,
        term: &str,
    ) -> bool {
        let last = (
            row as i32 + (term.len() - 1) as i32 * direction.values().0,
            column as i32 + (term.len() - 1) as i32 * direction.values().1,
        );

        if !self.is_within_bounds(last.0, last.1) {
            return false;
        }

        let mut next = (
            (row as i32 + direction.values().0) as usize,
            (column as i32 + direction.values().1) as usize,
        );

        for c in term.chars().skip(1) {
            if self.inner[next.0][next.1] != c {
                return false;
            }

            next = (
                (next.0 as i32 + direction.values().0) as usize,
                (next.1 as i32 + direction.values().1) as usize,
            );
        }

        true
    }

    fn is_within_bounds(&self, row: i32, column: i32) -> bool {
        if row < 0 || column < 0 {
            return false;
        }

        (row as usize) < self.inner.len() && (column as usize) < self.inner[0].len()
    }
}
