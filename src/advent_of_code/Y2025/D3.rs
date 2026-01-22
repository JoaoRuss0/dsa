pub fn run() {
    println!("  ├─ Day 3 - Secret Entrance");

    let path = "input/advent_of_code/Y2025/D3.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut joltage: u16 = 0;
    for batteries in banks {
        let mut one = 0;
        let mut two = 1;

        for i in 1..batteries.len() {
            if batteries[i] > batteries[one] && i != batteries.len() - 1 {
                one = i;
                two = i + 1;
                continue;
            }

            if batteries[i] > batteries[two] {
                two = i;
            }
        }
        joltage += batteries[one] as u16 * 10 + batteries[two] as u16;
    }

    println!("  │  ├─ Part 1: {}", joltage);
    //println!("  │  └─ Part 2: {}",);
}
