pub fn run() {
    println!("  ├─ Puzzle 6: Bird Spotters");

    let path = "input/flip_flop/Y2025/P6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let birds = input
        .lines()
        .map(|l| {
            let split = l.split_once(',').unwrap();
            (
                split.0.parse::<i32>().unwrap(),
                split.1.parse::<i32>().unwrap(),
            )
        })
        .map(|(x, y)| Bird::new(x, y))
        .collect::<Vec<Bird>>();

    let grid = Grid {
        height: 1000,
        width: 1000,
    };

    let mut simulation = Simulation::new(grid, birds);
    simulation.simulate(100);
    println!("  │  ├─ Part 1: {}", simulation.snap_picture());

    simulation.reset();
    println!(
        "  │  ├─ Part 2: {}",
        (0..1000)
            .map(|_| {
                simulation.simulate(3600);
                simulation.snap_picture() as u64
            })
            .sum::<u64>()
    );
    //println!("  │  └─ Part 3: {}", );
}

struct Simulation {
    grid: Grid,
    birds: Vec<Bird>,
}

impl Simulation {
    fn new(grid: Grid, birds: Vec<Bird>) -> Self {
        Self { grid, birds }
    }

    fn step(&mut self, steps: usize) {
        for bird in &mut self.birds {
            let mut new_position = bird.next_steps(steps as i32);
            self.grid.wrap(&mut new_position);
            bird.position = new_position;
        }
    }

    fn simulate(&mut self, steps: usize) {
        self.step(steps);
    }

    fn snap_picture(&self) -> usize {
        self.birds
            .iter()
            .filter(|b| self.grid.is_in_center(&b.position))
            .count()
    }

    fn reset(&mut self) {
        self.birds.iter_mut().for_each(Bird::reset);
    }
}

struct Grid {
    height: i32,
    width: i32,
}

impl Grid {
    fn is_in_center(&self, point: &Point) -> bool {
        let center_width = self.width / 2;
        let center_height = self.height / 2;

        let top_left_corner = Point {
            x: self.width / 4,
            y: self.height / 4,
        };

        point.x >= top_left_corner.x
            && point.x < top_left_corner.x + center_width
            && point.y >= top_left_corner.y
            && point.y < top_left_corner.y + center_height
    }

    fn wrap(&self, point: &mut Point) {
        point.x = point.x.rem_euclid(self.width);
        point.y = point.y.rem_euclid(self.height);
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&mut self, vector: &Vector, scalar: i32) {
        self.x = self.x + (vector.x * scalar);
        self.y = self.y + (vector.y * scalar);
    }
}

struct Vector {
    x: i32,
    y: i32,
}

struct Bird {
    position: Point,
    speed: Vector,
}

impl Bird {
    fn new(speed_x: i32, speed_y: i32) -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            speed: Vector {
                x: speed_x,
                y: speed_y,
            },
        }
    }

    fn next_steps(&mut self, scalar: i32) -> Point {
        let mut new = self.position.clone();
        new.add(&self.speed, scalar);
        new
    }

    fn reset(&mut self) {
        self.position = Point { x: 0, y: 0 };
    }
}
