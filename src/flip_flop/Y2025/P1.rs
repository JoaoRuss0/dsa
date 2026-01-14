pub fn run() {
    println!("  ├─ Prologue: Dream Vacation");

    let path = "input/flip_flop/Y2025/P1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let ba_na_ne_s: Vec<(usize, bool)> = input
        .lines()
        .map(|line| (line.len() / 2, line.contains("ne")))
        .collect();

    println!(
        "  │  ├─ Part 1: {}",
        ba_na_ne_s.iter().map(|(score, _)| score).sum::<usize>()
    );

    println!(
        "  │  ├─ Part 2: {}",
        ba_na_ne_s
            .iter()
            .filter(|(score, _)| score.is_multiple_of(2))
            .map(|(score, _)| score)
            .sum::<usize>()
    );

    println!(
        "  │  └─ Part 3: {}",
        ba_na_ne_s
            .iter()
            .filter(|(score, _)| score.is_multiple_of(2))
            .map(|(score, banena)| match banena {
                true => 0,
                false => *score,
            })
            .sum::<usize>()
    );
}
