pub fn run() {
    println!("  ├─ Day 1 - The Tyranny of the Rocket Equation");

    let path = "input/advent_of_code/Y2019/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let masses = input.lines().map(|line| line.parse::<u32>().unwrap());
    let fuel = masses.map(|m| fuel(m) as u32).collect::<Vec<u32>>();

    println!("  │  ├─ Part 1: {}", fuel.iter().sum::<u32>());
    println!(
        "  │  └─ Part 2: {}",
        fuel.into_iter().map(r_fuel).sum::<u32>()
    );
}

fn fuel(mass: u32) -> i32 {
    (mass as i32 / 3) - 2
}

fn r_fuel(mass: u32) -> u32 {
    if fuel(mass) <= 0 {
        return mass;
    }

    mass + r_fuel(fuel(mass) as u32)
}
