use std::collections::HashSet;

const INVALID_DOUBLE_CHARS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn run() {
    println!("  ├─ Day 5 - Doesn't He Have Intern-Elves For This?");

    let path = "input/advent_of_code/Y2015/D5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let valid = input
        .lines()
        .map(|w| {
            let mut two = HashSet::new();
            let mut vowels = 0;
            let mut double = false;

            let chars = w.chars().collect::<Vec<char>>();
            if VOWELS.contains(&chars[0]) {
                vowels += 1;
            }

            for i in 1..w.len() {
                two.insert(&w[i - 1..i + 1]);
                double |= chars[i] == chars[i - 1];
                if VOWELS.contains(&chars[i]) {
                    vowels += 1;
                }
            }

            !INVALID_DOUBLE_CHARS.iter().any(|&s| two.contains(s)) && vowels >= 3 && double
        })
        .filter(|&x| x)
        .count();

    println!("  │  ├─ Part 1: {}", valid);
    //println!("  │  └─ Part 2: {}", );
}
