use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Day 3 - Perfectly Spherical Houses in a Vacuum");

    let path = "input/advent_of_code/Y2015/D3.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let mut delivered = HashSet::new();
    delivered.insert((0, 0));

    let mut santa = (0, 0);
    for d in input.chars() {
        direct(d, &mut santa);
        delivered.insert(santa);
    }

    println!("  │  ├─ Part 1: {}", delivered.len());

    santa.0 = 0;
    santa.1 = 0;

    delivered.clear();
    delivered.insert((0, 0));

    let mut robot = (0, 0);
    for (i, d) in input.chars().enumerate() {
        match i % 2 {
            0 => {
                direct(d, &mut santa);
                delivered.insert(santa);
            }
            _ => {
                direct(d, &mut robot);
                delivered.insert(robot);
            }
        }
    }

    println!("  │  └─ Part 2: {}", delivered.len());
}

fn direct(d: char, (x, y): &mut (i32, i32)) {
    match d {
        '^' => *x += 1,
        'v' => *x -= 1,
        '>' => *y += 1,
        '<' => *y -= 1,
        _ => panic!("Invalid input"),
    };
}
