use std::collections::HashSet;

const INVALID_DOUBLE_CHARS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn run() {
    println!("  ├─ Day 5 - Doesn't He Have Intern-Elves For This?");

    let path = "input/advent_of_code/Y2015/D5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let niceness = input.lines().map(is_nice).collect::<Vec<(bool, bool)>>();
    println!(
        "  │  ├─ Part 1: {}",
        niceness.iter().filter(|&&(one, _)| one).count()
    );
    println!(
        "  │  └─ Part 2: {}",
        niceness.iter().filter(|&&(_, two)| two).count()
    );
}

fn is_nice(word: &str) -> (bool, bool) {
    let mut two = HashSet::new();
    let mut vowels = 0;
    let mut double = false;

    let mut twice = Vec::with_capacity(word.len() - 1);
    let mut is_twice = false;
    let mut xyx = false;

    let chars = word.chars().collect::<Vec<char>>();

    for i in 0..word.len() {
        if i >= 1 {
            let cc = &word[i - 1..i + 1];
            two.insert(cc);
            double |= chars[i] == chars[i - 1];

            if !is_twice {
                is_twice |= twice[0..i - 1].contains(&cc);
                twice.push(cc);
            }
        }
        if VOWELS.contains(&chars[i]) {
            vowels += 1;
        }

        if i >= 2 && !xyx {
            let aba = &chars[i - 2..i + 1];
            xyx |= aba[0] == aba[2];
        }
    }

    (
        !INVALID_DOUBLE_CHARS.iter().any(|&s| two.contains(s)) && vowels >= 3 && double,
        is_twice && xyx,
    )
}
