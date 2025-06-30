pub fn run() {
    println!("  ├─ Day 1 - The Tyranny of the Rocket Equation");

    let path = "input/advent_of_code/Y2019/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let masses = input.lines().map(|line| line.parse::<u32>().unwrap());
    let fuel = masses.map(|m| m / 3 - 2).sum::<u32>();

    println!("  │  ├─ Part 1: {}", fuel);
    //println!("  │  └─ Part 2: {}", );
}
