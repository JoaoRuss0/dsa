pub fn run() {
    println!("  ├─ Puzzle 4: Beach Cleanup");

    let path = "input/flip_flop/Y2025/P4.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let trash = input
        .lines()
        .map(|l| {
            let split = l.split_once(",").unwrap();
            (
                split.0.parse::<u8>().unwrap(),
                split.1.parse::<u8>().unwrap(),
            )
        })
        .collect::<Vec<(u8, u8)>>();

    let mut steps: u64 = 0;
    let mut last = (0, 0);
    for t in &trash {
        steps += (t.0.abs_diff(last.0) + t.1.abs_diff(last.1)) as u64;
        last = t.clone();
    }

    println!("  │  ├─ Part 1: {}", steps);

    let mut steps: u64 = 0;
    let mut last = (0, 0);
    for t in trash {
        let vector = (t.0.abs_diff(last.0), t.1.abs_diff(last.1));
        let diff = vector.0.abs_diff(vector.1);

        match diff == 0 {
            true => steps += vector.0 as u64,
            false => steps += diff as u64 + vector.0.min(vector.1) as u64,
        }

        last = t
    }

    println!("  │  ├─ Part 2: {}", steps);
    //println!("  │  └─ Part 3: {}",);
}
