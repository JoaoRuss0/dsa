pub fn run() {
    println!("  ├─ Day 1 - Trebuchet?!");

    let path = "input/advent_of_code/Y2023/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut sum = input
        .lines()
        .map(|l| calibrate(l, convert_digit))
        .sum::<u32>();
    println!("  │  ├─ Part 1: {sum}");

    sum = input.lines().map(|l| calibrate(l, convert)).sum::<u32>();
    println!("  │  └─ Part 2: {sum}");
}

fn calibrate(l: &str, convert: fn(char, &str) -> Option<u32>) -> u32 {
    let mut first = None;
    let mut last = None;

    let chars = l.chars().collect::<Vec<char>>();
    for i in 0..l.len() {
        let digit = convert(chars[i], &l[i..]);
        if digit.is_none() {
            continue;
        }
        if first.is_none() {
            first = digit;
        } else {
            last = digit;
        }
    }

    first.unwrap() * 10 + last.or(first).unwrap()
}

fn convert_digit(c: char, _: &str) -> Option<u32> {
    match c.is_ascii_digit() {
        true => Some(c.to_digit(10).unwrap()),
        false => None,
    }
}

fn convert(c: char, l: &str) -> Option<u32> {
    match c.is_ascii_digit() {
        true => c.to_digit(10),
        false => Digit::from_str(l),
    }
}

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn entries() -> &'static [Digit] {
        static DIGITS: [Digit; 9] = [
            Digit::One,
            Digit::Two,
            Digit::Three,
            Digit::Four,
            Digit::Five,
            Digit::Six,
            Digit::Seven,
            Digit::Eight,
            Digit::Nine,
        ];
        &DIGITS
    }

    fn string(&self) -> &'static str {
        match self {
            Digit::One => "one",
            Digit::Two => "two",
            Digit::Three => "three",
            Digit::Four => "four",
            Digit::Five => "five",
            Digit::Six => "six",
            Digit::Seven => "seven",
            Digit::Eight => "eight",
            Digit::Nine => "nine",
        }
    }

    fn value(&self) -> u32 {
        match self {
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }

    fn from_str(l: &str) -> Option<u32> {
        for d in Digit::entries() {
            if l.starts_with(d.string()) {
                return Some(d.value());
            }
        }
        None
    }
}
