pub fn run() {
    println!("  ├─ Day 1 - Trebuchet?!");

    let path = "input/advent_of_code/Y2023/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let sum = input
        .lines()
        .map(|l| {
            let mut first = None;
            let mut last = None;

            for i in l.chars() {
                if i.is_ascii_digit() {
                    let digit = i.to_digit(10).unwrap();
                    if first.is_none() {
                        first = Some(digit);
                    } else {
                        last = Some(digit);
                    }
                }
            }

            first.unwrap() * 10 + last.or(first).unwrap()
        })
        .sum::<u32>();

    println!("  │  ├─ Part 1: {}", sum);
    //println!("  │  └─ Part 2: {}", );
}
