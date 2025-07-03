pub fn run() {
    println!("  ├─ Day 1 - Not Quite Lisp");

    let path = "input/advent_of_code/Y2015/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let chars = input.chars();

    let directions = chars
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<i32>>();

    let mut sum = 0;
    let mut p_basement: usize = 0;
    let mut has_entered_basement = false;

    for (i, d) in directions.iter().enumerate() {
        sum += d;

        if !has_entered_basement && sum < 0 {
            p_basement = i;
            has_entered_basement = true;
        }
    }

    println!("  │  ├─ Part 1: {sum}");
    println!("  │  └─ Part 2: {}", p_basement + 1);
}
