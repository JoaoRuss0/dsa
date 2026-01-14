pub fn run() {
    println!("  ├─ Prologue: Dream Vacation");

    let path = "input/flip_flop/Y2025/P1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let ba_na_ne_s: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, calculate_banana_score(line)))
        .collect();

    println!(
        "  │  ├─ Part 1: {}",
        ba_na_ne_s.iter().map(|(_, score)| score).sum::<usize>()
    );

    println!(
        "  │  └─ Part 2: {}",
        ba_na_ne_s
            .iter()
            .filter(|(_, score)| score.is_multiple_of(2))
            .map(|(_, score)| score)
            .sum::<usize>()
    );
}

pub fn calculate_banana_score(line: &str) -> usize {
    let ba_na_ne_s: Vec<&str> = vec!["ba", "na", "ne"];

    let mut score = 0;
    let mut pair = vec![];
    let mut odd = true;

    for char in line.chars() {
        if odd {
            odd = false;
            pair.push(char);
            continue;
        }
        pair.push(char);

        print!("{} ", pair.iter().collect::<String>());
        if ba_na_ne_s.contains(&pair.iter().collect::<String>().as_str()) {
            score += 1;
        }

        pair.clear();
        odd = true;
    }

    println!("{}", score);
    score
}
