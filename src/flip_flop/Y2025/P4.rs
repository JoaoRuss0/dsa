pub fn run() {
    println!("  ├─ Puzzle 4: Beach Cleanup");

    let path = "input/flip_flop/Y2025/P4.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let trash = input
        .lines()
        .map(|l| {
            let split = l.split_once(",").unwrap();
            Point {
                x: split.0.parse::<u8>().unwrap(),
                y: split.1.parse::<u8>().unwrap(),
            }
        })
        .collect::<Vec<Point>>();

    let mut steps = walk(&trash, |x, y| x.step(y));
    println!("  │  ├─ Part 1: {}", steps);

    steps = walk(&trash, |x, y| x.step_diagonally(y));
    println!("  │  ├─ Part 2: {}", steps);

    //println!("  │  └─ Part 3: {}",);
}

struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn step(&self, next: &Point) -> u8 {
        self.x.abs_diff(next.x) + self.y.abs_diff(next.y)
    }

    fn step_diagonally(&self, next: &Point) -> u8 {
        let vector = Point {
            x: self.x.abs_diff(next.x),
            y: self.y.abs_diff(next.y),
        };
        let diff = vector.x.abs_diff(vector.y);

        match diff == 0 {
            true => vector.x,
            false => diff + vector.x.min(vector.y),
        }
    }
}

fn walk(trash: &Vec<Point>, count_steps: fn(&Point, &Point) -> u8) -> u64 {
    let mut steps: u64 = 0;
    let mut last = &Point { x: 0, y: 0 };

    for t in trash {
        steps += count_steps(t, last) as u64;
        last = t;
    }

    steps
}
