pub fn run() {
    println!("  ├─ Problem 8 - Risky Shortcut");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem8.txt").unwrap();

    let lines = input.lines().collect::<Vec<&str>>();

    let composition = |line: &str| -> usize { line.chars().filter(|c| c.is_alphabetic()).count() };

    println!(
        "  │  ├─ Part 1: {}",
        lines.iter().map(|&l| composition(l)).sum::<usize>()
    );

    println!(
        "  │  ├─ Part 2: {}",
        lines
            .iter()
            .map(|&l| reduce(l, can_reduce_hyphenated).len())
            .sum::<usize>()
    );

    println!(
        "  │  └─ Part 3: {}",
        lines
            .iter()
            .map(|&l| reduce(l, can_reduce).len())
            .sum::<usize>()
    );
}

fn can_reduce_hyphenated(c1: char, c2: char) -> bool {
    c1.is_numeric() && (c2.is_alphabetic() || c2 == '-')
        || c2.is_numeric() && (c1.is_alphabetic() || c1 == '-')
}

fn can_reduce(c1: char, c2: char) -> bool {
    c1.is_numeric() && c2.is_alphabetic() || c2.is_numeric() && c1.is_alphabetic()
}

fn reduce(line: &str, can: fn(char, char) -> bool) -> String {
    let mut chars = line.chars().collect::<Vec<char>>();
    let mut temp = String::new();

    let mut reduced;
    loop {
        reduced = false;
        let mut i = 0;
        while i < chars.len() {
            if i < chars.len() - 1 && can(chars[i], chars[i + 1]) {
                i += 2;
                reduced = true;
                continue;
            }

            temp.push(chars[i]);
            i += 1;
        }
        chars = temp.chars().collect();
        temp.clear();
        if !reduced {
            break;
        }
    }
    chars.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("ba", reduce("baa3", can_reduce_hyphenated));
    }

    #[test]
    fn test2() {
        assert_eq!("3", reduce("321ab", can_reduce_hyphenated));
    }

    #[test]
    fn test3() {
        assert_eq!("b", reduce("a7b", can_reduce_hyphenated));
    }

    #[test]
    fn test4() {
        assert_eq!("z", reduce("z-4", can_reduce_hyphenated));
    }

    #[test]
    fn test5() {
        assert_eq!("ba", reduce("baa3", can_reduce));
    }

    #[test]
    fn test6() {
        assert_eq!("3", reduce("321ab", can_reduce));
    }

    #[test]
    fn test7() {
        assert_eq!("b", reduce("a7b", can_reduce));
    }

    #[test]
    fn test8() {
        assert_eq!("z-4", reduce("z-4", can_reduce));
    }
}
