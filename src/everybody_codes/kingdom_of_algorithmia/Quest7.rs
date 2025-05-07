use std::collections::HashMap;

pub fn run() {
    println!("  ├─ Quest 7 - Not Fast but Furious");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/quest7/part1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();
    let mut devices = get_devices(input);
    let mut track = vec!['='];

    println!(
        "  │  ├─ Part 1: {}",
        rank_device_plans(&devices, &track, 10)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/quest7/part2.txt";
    input = std::fs::read_to_string(path).unwrap();
    devices = get_devices(input);

    path = "input/everybody_codes/kingdom_of_algorithmia/quest7/part2_track.txt";
    input = std::fs::read_to_string(path).unwrap();
    track = get_track(input);
    println!(
        "  │  ├─ Part 2: {}",
        rank_device_plans(&devices, &track, 10)
    );

    //path = "input/everybody_codes/kingdom_of_algorithmia/quest7/part3.txt";
    //input = std::fs::read_to_string(path).unwrap();
    //println!("  │  └─ Part 3: {}", );
}

fn get_devices(input: String) -> HashMap<String, Vec<char>> {
    input
        .lines()
        .map(|l| {
            let (id, raw_actions) = l.split_once(":").unwrap();
            (
                String::from(id),
                raw_actions
                    .split(",")
                    .map(|a| a.chars().next().unwrap())
                    .collect::<Vec<char>>(),
            )
        })
        .collect::<HashMap<String, Vec<char>>>()
}

fn get_track(input: String) -> Vec<char> {
    let raw_track = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut track: Vec<char> = Vec::new();

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut pos: (usize, usize) = (0, 0);
    for d in directions {
        loop {
            let next_candidate: (i32, i32) = (pos.0 as i32 + d.0, pos.1 as i32 + d.1);
            if next_candidate.0 < 0 || next_candidate.1 < 0 {
                break;
            }

            let next = (next_candidate.0 as usize, next_candidate.1 as usize);
            if next.0 >= raw_track.len() || next.1 >= raw_track[next.0].len() {
                break;
            }

            track.push(raw_track[next.0][next.1]);
            pos = next
        }
    }

    track
}

fn rank_device_plans(devices: &HashMap<String, Vec<char>>, track: &[char], laps: usize) -> String {
    let mut ranking = devices
        .iter()
        .map(|(id, plan)| {
            let mut power = 10;
            let mut sum = 0;
            let mut pos = 0;
            (0..laps).for_each(|_| sum += run_track(track, plan, &mut power, &mut pos));
            (id.clone(), sum)
        })
        .collect::<Vec<(String, usize)>>();
    ranking.sort_by(|p, n| n.1.cmp(&p.1));
    ranking.iter().map(|(id, _)| id.clone()).collect::<String>()
}

fn run_track(track: &[char], plan: &[char], power: &mut usize, pos: &mut usize) -> usize {
    let mut sum = 0;

    for &step in track {
        let action = if step == '=' || step == 'S' {
            plan[*pos % plan.len()]
        } else {
            step
        };

        match action {
            '+' => *power += 1,
            '-' => {
                if *power > 0 {
                    *power -= 1
                }
            }
            _ => (),
        };
        sum += *power;
        *pos += 1;
    }
    sum
}
