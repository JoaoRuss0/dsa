pub fn run() {
    println!("  ├─ Day 1 - Secret Entrance");

    let path = "input/advent_of_code/Y2025/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let rotations = input.lines().map(Rotation::from).collect::<Vec<Rotation>>();

    let add = |curr: usize, amount: usize| -> (usize, usize) {
        let mut next = curr + amount;
        let mut loops = 0;

        if next >= 100 {
            loops += next / 100;
            next %= 100;

            if next == 100 {
                next = 0;
            }
        };

        (next, loops)
    };

    let sub = |curr: usize, amount: usize| -> (usize, usize) {
        let mut next = 100 - curr + amount;
        let mut loops = 0;
        let from_zero = curr == 0;

        match next >= 100 {
            true => {
                loops += next / 100;
                if from_zero {
                    loops -= 1;
                }
                next = 100 - (next % 100);
                if next == 100 {
                    next = 0;
                }
            }
            false => {
                next = 100 - next;
            }
        }

        (next, loops)
    };

    let mut at_zero: usize = 0;
    let mut loops: usize = 0;
    let mut curr: usize = 50;
    for r in rotations {
        let (next, inner_loops) = match r.direction {
            Direction::Left => sub(curr, r.amount),
            Direction::Right => add(curr, r.amount),
        };

        if next == 0 {
            at_zero += 1;
        }
        loops += inner_loops;
        curr = next;
    }

    println!("  │  ├─ Part 1: {}", at_zero);
    println!("  │  └─ Part 2: {}", loops);
}

#[derive(Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Rotation {
    direction: Direction,
    amount: usize,
}

impl Rotation {
    fn from(string: &str) -> Rotation {
        let chars = string.chars().collect::<Vec<char>>();
        let direction: Direction = chars[0].into();
        let amount = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Rotation { direction, amount }
    }
}
