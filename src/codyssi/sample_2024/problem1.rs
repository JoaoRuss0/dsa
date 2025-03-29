pub fn run() {

    println!("Problem 1");

    let input = std::fs::read_to_string("input/codyssi/sample_2024/problem1.txt")
        .unwrap();

    let mut prices = input.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let discount = prices.iter()
        .enumerate()
        .map(|(i, a)| {
            match i % 2 == 0 {
                true => *a as i64,
                false => -(*a as i64),
            }
        })
        .sum::<i64>();

    prices.sort_by(|a, b| b.cmp(a));

    println!("- Part 1: {}", prices.iter().sum::<u32>());
    println!("- Part 2: {}", prices.iter().skip(20).sum::<u32>());
    println!("- Part 3: {}", discount);
}