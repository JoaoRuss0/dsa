pub fn run() {
    println!("  ├─ Prologue: Dream Vacation");

    let path = "input/flip_flop/Y2025/P1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let ba_na_ne_s = input
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|pair| vec!["ba".to_string(), "na".to_string(), "ne".to_string()].contains(pair))
        .count();

    println!("  │  ├─ Part 1: {}", ba_na_ne_s);
    //println!("  │  └─ Part 2: {}",);
}
