pub fn run() {
    println!("  ├─ Day 1 - Calorie Counting");

    let path = "input/advent_of_code/Y2022/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let inventories = input.split("\n\n").collect::<Vec<&str>>();
    let calories = inventories
        .iter()
        .map(|i| {
            i.lines()
                .map(|l| l.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!(
        "  │  ├─ Part 1: {}",
        calories
            .iter()
            .map(|c| c.iter().sum::<u32>())
            .max()
            .unwrap()
    );
    //println!("  │  └─ Part 2: {}", );
}
