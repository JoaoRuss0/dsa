pub fn run() {
    println!("  ├─ Day 1 - Calorie Counting");

    let path = "input/advent_of_code/Y2022/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let inventories = input.split("\n\n").collect::<Vec<&str>>();
    let mut calories = inventories
        .iter()
        .map(|i| i.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    println!("  │  ├─ Part 1: {}", calories.iter().max().unwrap());

    calories.sort_by(|a, b| b.cmp(a));
    println!(
        "  │  └─ Part 2: {}",
        calories[0] + calories[1] + calories[2]
    );
}
