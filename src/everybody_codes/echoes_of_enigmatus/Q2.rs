pub fn run() {
    println!("  ├─ Quest 2 - Tangles Trees");

    let mut path = "input/everybody_codes/echoes_of_enigmatus/Q2/P1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    let commands = get_commands(input);
    println!("  │  ├─ Part 1: {}", 1);

    //path = "input/everybody_codes/echoes_of_enigmatus/Q2/P2.txt";
    //input = std::fs::read_to_string(path).unwrap();

    //println!("  │  ├─ Part 2: {}", );

    //path = "input/everybody_codes/echoes_of_enigmatus/Q2/P3.txt";
    //input = std::fs::read_to_string(path).unwrap();
    //println!("  │  └─ Part 3: {}", );
}

#[derive(Debug)]
struct Command {
    id: usize,
    left: (usize, char),
    right: (usize, char),
}

fn get_commands(input: String) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let split = l.split_whitespace().collect::<Vec<&str>>();

            let id = split[1].split("=").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let n_left = split[2].split("=").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>();
            let n_right = split[3].split("=").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>();

            let l_val = n_left[0][1..].parse::<usize>().unwrap();
            let l_id = n_left[1][0..n_left[1].len() - 1].chars().next().unwrap();

            let r_val = n_right[0][1..].parse::<usize>().unwrap();
            let r_id = n_right[1][0..n_right[1].len() - 1].chars().next().unwrap();

            Command {
                id,
                left: (l_val, l_id),
                right: (r_val, r_id),
            }
        })
        .collect()
}
