pub fn run() {
    println!("  ├─ Problem 11 - Games in a Storm");

    let path = "input/codyssi/journey_to_atlantis/problem11.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let numbers_and_base = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(a, b)| (a, b.parse::<u64>().unwrap()))
        .collect::<Vec<(&str, u64)>>();

    let base_10 = numbers_and_base
        .iter()
        .map(|&(number, base)| convert_from_custom_base(number, base))
        .collect::<Vec<u64>>();

    println!("  │  ├─ Part 1: {}", base_10.iter().max().unwrap());

    let sum = base_10.iter().sum();
    println!("  │  ├─ Part 2: {}", convert_to_custom_base(sum, 68));

    println!("  │  └─ Part 3: {}", find_smallest_4_digit_base(sum));
}

pub fn find_smallest_4_digit_base(number: u64) -> u64 {
    let mut base = 1;
    loop {
        let mut divided = number;

        (0..4).for_each(|_| {
            divided /= base;
        });

        if divided == 0 {
            return base;
        }
        base += 1;
    }
}

pub fn convert_to_custom_base(number: u64, base: u64) -> String {
    const BASE: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^";

    let mut r: Vec<char> = Vec::new();

    let mut divided = number;
    while divided > 0 {
        let remainder = divided % base;
        divided /= base;
        r.push(BASE.chars().nth(remainder as usize).unwrap());
    }
    r.reverse();
    r.iter().collect::<String>()
}

pub fn convert_from_custom_base(number: &str, base: u64) -> u64 {
    const BASE: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^";

    let mut r: u64 = 0;
    let mut position = base.pow((number.len() - 1) as u32);
    for i in (0..number.len()).rev() {
        let digit = number.chars().nth(number.len() - 1 - i).unwrap();
        let value = BASE.find(digit).unwrap() as u64;
        r += value * position;
        position /= base;
    }

    r
}
