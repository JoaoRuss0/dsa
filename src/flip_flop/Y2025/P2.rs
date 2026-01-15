use std::collections::HashMap;

pub fn run() {
    println!("  ├─ Puzzle 2: Rollercoaster Heights");

    let path = "input/flip_flop/Y2025/P2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let movements: Vec<char> = input.chars().collect();
    let peaks = calculate_peaks(movements);

    println!(
        "  │  ├─ Part 1: {}",
        peaks.iter().map(|p| p.0).max().unwrap()
    );
    println!(
        "  │  ├─ Part 2: {}",
        peaks.iter().map(|p| p.1).max().unwrap()
    );
    println!(
        "  │  └─ Part 3: {}",
        peaks.iter().map(|p| p.2).max().unwrap()
    );
}

fn calculate_peaks(movements: Vec<char>) -> Vec<(i64, i64, i64)> {
    let mut height: i64 = 0;
    let mut higher_height: i64 = 0;
    let mut fib_score: i64 = 0;
    let mut fib = Fibonacci::new();

    let mut peaks: Vec<(i64, i64, i64)> = Vec::new();

    let get_signal = |m: char| match m {
        'v' => -1,
        '^' => 1,
        _ => panic!("Invalid movement: {}", m),
    };

    let mut last = (movements[0], 0);

    for (i, &movement) in movements.iter().enumerate() {
        let signal = get_signal(movement);
        let times: i64 = match last.0 == movement {
            true => last.1 + 1,
            false => 1,
        };

        let to_add = match last.0 != movement {
            true => get_signal(last.0) * fib.get(last.1 as u16) as i64,
            false => 0,
        };

        height += signal;
        higher_height += times * signal;
        fib_score = fib_score.checked_add(to_add).unwrap();

        peaks.push((height, higher_height, fib_score));
        last = (movement, times);
    }
    peaks
}

struct Fibonacci {
    cache: HashMap<u16, u32>,
}

impl Fibonacci {
    fn new() -> Self {
        let mut f = Self {
            cache: HashMap::new(),
        };

        f.cache.insert(1, 1);
        f.cache.insert(2, 1);

        f
    }

    fn get(&mut self, value: u16) -> u32 {
        if self.cache.contains_key(&value) {
            return self.cache[&value];
        }

        let prev = self.get(value - 1);
        let prev_prev = self.get(value - 2);

        self.cache.insert(value, prev + prev_prev);
        self.cache[&value]
    }
}
