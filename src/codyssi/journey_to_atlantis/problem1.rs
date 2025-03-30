pub fn run() {
    println!("  ├─ Problem 1 - Compass Calibration");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem1.txt")
        .unwrap();

    let lines = input.lines().collect::<Vec<&str>>();
    let offset = lines.get(0)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let operations = lines.get(lines.len() - 1).unwrap();

    let mut corrected = offset;
    for i in 1..lines.len() - 1 {
        let correction = operations.chars().nth(i - 1).unwrap().to_string() + &lines[i];
        corrected += correction.parse::<i64>().unwrap();
    }

    println!("  │  ├─ Part 1: {}", corrected);

    let mut corrected_rev = offset;
    for i in (0..=operations.len() - 1).rev() {
        let correction = operations.chars().nth(i).unwrap().to_string()
            + &lines[operations.len() - i];
        corrected_rev += correction.parse::<i64>().unwrap();
    }

    println!("  │  ├─ Part 2: {}", corrected_rev);

    let mut corrected_rev_two_digit = (lines[0].to_string() + lines[1]).parse::<i64>().unwrap();

    for i in 1..=operations.len()/2 {
        let correction = operations.chars().nth(operations.len() - i).unwrap().to_string() +
            &lines[i * 2] +
            &lines[i * 2 + 1];
        corrected_rev_two_digit += correction.parse::<i64>().unwrap();
    }

    println!("  │  └─ Part 3: {}", corrected_rev_two_digit);
}