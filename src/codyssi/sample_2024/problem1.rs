pub fn run() {

    let input = std::fs::read_to_string("input/codyssi/sample_2024/problem1.txt")
        .unwrap();
    let mut prices = input.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    prices.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", prices.iter().sum::<u64>());
    println!("Part 2: {}", prices.iter().skip(20).sum::<u64>());
}