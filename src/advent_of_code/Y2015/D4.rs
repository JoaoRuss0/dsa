pub fn run() {
    println!("  ├─ Day 4 - The Ideal Stocking Stuffer");

    let path = "input/advent_of_code/Y2015/D4.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let key = input.lines().next().unwrap();
    let mut i = 0;

    let mut five = None;
    let mut six = None;

    loop {
        let input = key.to_string() + i.to_string().as_str();
        let hash = md5::compute(&input);

        if six.is_none() && hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            six = Some(i);
        }

        if five.is_none() && hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
            five = Some(i);
        }

        if five.is_some() && six.is_some() {
            break;
        }

        i += 1;
    }

    println!("  │  ├─ Part 1: {}", five.unwrap());
    println!("  │  └─ Part 2: {}", six.unwrap());
}
