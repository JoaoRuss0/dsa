pub fn run() {
    println!("  ├─ Day 1 - No Time for a Taxicab");

    let path = "input/advent_of_code/Y2016/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let instructions = input
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|s| {
            let mut chars = s.chars().collect::<Vec<char>>();
            let mut num = chars[1..]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            (chars[0], num)
        })
        .collect::<Vec<(char, u32)>>();

    let mut directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction_idx: usize = 0;
    let mut pos = (0i32, 0i32);
    instructions.iter().for_each(|&(dir, num)| {
        direction_idx = (direction_idx as i32
            + match dir {
                'R' => 1,
                'L' => -1,
                _ => panic!("Not a valid rotation"),
            })
        .rem_euclid(4) as usize;

        pos = (
            pos.0 + (directions[direction_idx].0 * num as i32),
            pos.1 + (directions[direction_idx].1 * num as i32),
        );
    });

    println!("  │  ├─ Part 1: {}", pos.0.abs() + pos.1.abs());
    //println!("  │  └─ Part 2: {}", );
}
