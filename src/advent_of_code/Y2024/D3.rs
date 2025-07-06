use regex::Regex;

pub fn run() {
    println!("  ├─ Day 3 - Mull It Over");

    let path = "input/advent_of_code/Y2024/D3.txt";
    let input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  ├─ Part 1: {}",
        parse(&input).iter().map(|(m, n)| m * n).sum::<u32>()
    );

    println!(
        "  │  └─ Part 2: {}",
        parse_enabled(&input)
            .iter()
            .map(|(m, n)| m * n)
            .sum::<u32>()
    );
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex
        .captures_iter(&input)
        .map(|c| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()))
        .collect()
}

fn parse_enabled(input: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut instructions = Vec::new();
    let mut enabled = true;

    for captured in regex.captures_iter(&input) {
        match captured[0].to_string().as_str() {
            "don't()" => {
                enabled = false;
                continue;
            }
            "do()" => {
                enabled = true;
                continue;
            }
            _ => {
                if !enabled {
                    continue;
                }
            }
        }

        instructions.push((
            captured[1].parse::<u32>().unwrap(),
            captured[2].parse::<u32>().unwrap(),
        ))
    }

    instructions
}
