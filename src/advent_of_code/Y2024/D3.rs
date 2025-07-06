use regex::Regex;

pub fn run() {
    println!("  ├─ Day 3 - Mull It Over");

    let path = "input/advent_of_code/Y2024/D3.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let muls = regex
        .captures_iter(&input)
        .map(|captured| {
            (
                captured[1].parse::<u32>().unwrap(),
                captured[2].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();

    println!(
        "  │  ├─ Part 1: {}",
        muls.iter().map(|(m, n)| m * n).sum::<u32>()
    );
    //println!("  │  └─ Part 2: {}", );
}
