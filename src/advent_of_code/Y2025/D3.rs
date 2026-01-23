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

    println!(
        "  │  ├─ Part 1: {}",
        banks
            .iter()
            .map(|b| calculate_max_joltage(b, 2))
            .sum::<usize>()
    );
    println!(
        "  │  └─ Part 2: {}",
        banks
            .iter()
            .map(|b| { calculate_max_joltage(b, 12) })
            .sum::<usize>()
    );
}

fn calculate_max_joltage(bank: &Vec<u8>, quantity: usize) -> usize {
    let battery_count = bank.len();

    let mut selected = vec![0; quantity];
    for i in 0..quantity {
        selected[i] = i;
    }

    for i in 1..=(bank.len() - quantity) {
        for (j, &battery) in selected.iter().enumerate() {
            let subset_start_index = i + j;
            let subset_quantity = quantity - j;
            let remaining = battery_count - i;

            if bank[battery] < bank[subset_start_index] && remaining >= subset_quantity {
                (j..quantity)
                    .enumerate()
                    .for_each(|(k, index)| selected[index] = subset_start_index + k);
                break;
            }
        }
    }

    (0..quantity)
        .rev()
        .enumerate()
        .map(|(i, bank_index)| bank[selected[bank_index]] as usize * 10_usize.pow(i as u32))
        .sum()
}
