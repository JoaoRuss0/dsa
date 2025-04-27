pub fn run() {
    println!("  ├─ Quest 4 - Royal Smith's Puzzle");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/quest4/part1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    let mut nails = input
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut sum = nails.iter().sum::<usize>();
    let mut min = nails.iter().min().unwrap();
    println!("  │  ├─ Part 1: {}", sum - min * nails.len());

    path = "input/everybody_codes/kingdom_of_algorithmia/quest4/part2.txt";
    input = std::fs::read_to_string(path).unwrap();

    nails = input
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    sum = nails.iter().sum::<usize>();
    min = nails.iter().min().unwrap();
    println!("  │  ├─ Part 2: {}", sum - min * nails.len());

    path = "input/everybody_codes/kingdom_of_algorithmia/quest4/part3.txt";
    input = std::fs::read_to_string(path).unwrap();

    nails = input
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    nails.sort();
    let median = *nails.get(nails.len() / 2).unwrap();
    println!(
        "  │  └─ Part 3: {}",
        nails.iter().map(|&n| n.abs_diff(median)).sum::<usize>()
    );
}
