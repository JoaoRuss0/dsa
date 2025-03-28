pub fn run() {

    let input = std::fs::read_to_string("input/codyssi/sample_2024/problem1.txt")
        .unwrap();
    let sum = input.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .sum::<u64>();
    println!("Part 1: {}", sum);
}