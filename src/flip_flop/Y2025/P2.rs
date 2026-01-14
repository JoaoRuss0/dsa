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
    //println!("  │  └─ Part 3: {}",);
}

fn calculate_peaks(movements: Vec<char>) -> Vec<(i64, i64)> {
    let mut height = 0;
    let mut higher_height = 0;

    let mut peaks = Vec::new();

    let mut last = ('-', 1);
    for movement in movements {
        let amount = match movement {
            'v' => -1,
            '^' => 1,
            _ => panic!("Invalid movement: {}", movement),
        };

        let times = match last.0 == movement {
            true => last.1 + 1,
            false => 1,
        };

        height += amount;
        higher_height += times * amount;

        peaks.push((height, higher_height));
        last = (movement, times);
    }
    peaks
}
