pub fn run() {
    println!("  ├─ Quest 1 - The Battle for the Farmlands");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/quest1/part1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  ├─ Part 1: {}",
        input.chars().map(get_enemy_potion_cost).sum::<u64>()
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/quest1/part2.txt";
    input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  ├─ Part 2: {}",
        input
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(get_battle_cost)
            .sum::<u64>()
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/quest1/part3.txt";
    input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  └─ Part 3: {}",
        input
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(get_battle_cost)
            .sum::<u64>()
    );
}

fn get_enemy_potion_cost(enemy: char) -> u64 {
    match enemy {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => panic!("Invalid character"),
    }
}

fn get_battle_cost(enemies: &[char]) -> u64 {
    let empty = enemies.iter().map(|&e| e == 'x').filter(|&b| b).count();
    if empty == enemies.len() {
        return 0;
    }

    if empty >= 1 {
        let cost = enemies
            .iter()
            .filter(|&&e| e != 'x')
            .map(|&e| get_enemy_potion_cost(e))
            .sum::<u64>();

        return cost + ((enemies.len() - empty - 1) * (enemies.len() - 1)) as u64;
    }

    enemies
        .iter()
        .map(|&e| get_enemy_potion_cost(e))
        .sum::<u64>()
        + (enemies.len() * (enemies.len() - 1)) as u64
}
