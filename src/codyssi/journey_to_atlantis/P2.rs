pub fn run() {
    println!("  ├─ Problem 2 - Absurd Arithmetic");

    let path = "input/codyssi/journey_to_atlantis/P2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut rooms = input
        .lines()
        .skip(4)
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    rooms.sort();

    let apply_functions = |mut quality: i64| -> i64 {
        quality = quality.pow(3);
        quality *= 83;
        quality += 218;
        quality
    };

    println!(
        "  │  ├─ Part 1: {}",
        apply_functions(*rooms.get(rooms.len() / 2).unwrap())
    );
    println!(
        "  │  ├─ Part 2: {}",
        apply_functions(rooms.iter().filter(|&q| q % 2 == 0).sum::<i64>())
    );
    println!(
        "  │  └─ Part 3: {}",
        rooms
            .iter()
            .filter(|&&q| apply_functions(q) < 15000000000000)
            .max()
            .unwrap()
    );
}
