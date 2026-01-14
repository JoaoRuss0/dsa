pub fn run() {
    println!("  ├─ Puzzle 2: Rollercoaster Heights");

    let path = "input/flip_flop/Y2025/P2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let movements: Vec<char> = input.chars().collect();
    let peaks = calculate_peaks(movements);

    println!("  │  ├─ Part 1: {}", peaks.iter().max().unwrap());
    //println!("  │  ├─ Part 2: {}",);
    //println!("  │  └─ Part 3: {}",);
}

fn calculate_peaks(movements: Vec<char>) -> Vec<i64> {
    let mut height = 0;
    let mut peaks = Vec::new();

    for movement in movements {
        height += match movement {
            'v' => -1,
            '^' => 1,
            _ => panic!("Invalid movement: {}", movement),
        };
        peaks.push(height);
    }

    peaks
}
