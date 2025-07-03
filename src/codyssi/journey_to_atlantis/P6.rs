pub fn run() {
    println!("  ├─ Problem 6 - Lotus Scramble");

    let path = "input/codyssi/journey_to_atlantis/P6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let line = input.lines().next().unwrap();

    let uncorrupted = line
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    println!("  │  ├─ Part 1: {}", uncorrupted.len());

    let value = |&c: &char| -> u64 {
        match c {
            'a'..='z' => ((c as u8) - b'a' + 1) as u64,
            'A'..='Z' => ((c as u8) - b'A' + 27) as u64,
            _ => 0,
        }
    };

    let sum: u64 = uncorrupted.iter().map(value).sum();

    println!("  │  ├─ Part 2: {sum}");

    let correct = |previous_value: u64| -> u64 {
        let corrected: i64 = previous_value as i64 * 2 - 5;
        match corrected {
            1..=52 => corrected as u64,
            _ => corrected.rem_euclid(52) as u64,
        }
    };

    let chars = line.chars();
    let mut original_value = 0;
    let mut previous_value = 0;

    for c in chars {
        let new_value = match c.is_alphabetic() {
            true => value(&c),
            false => correct(previous_value),
        };
        original_value += new_value;
        previous_value = new_value;
    }

    println!("  │  └─ Part 3: {original_value}");
}
