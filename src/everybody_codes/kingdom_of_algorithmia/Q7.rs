use std::collections::HashMap;

pub fn run() {
    println!("  ├─ Quest 7 - Not Fast but Furious");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/Q7/P1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();
    let mut devices = get_devices(input);
    let mut track = vec!['='];

    println!(
        "  │  ├─ Part 1: {}",
        rank_device_plans(&devices, &track, 10)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/Q7/P2.txt";
    input = std::fs::read_to_string(path).unwrap();
    devices = get_devices(input);

    path = "input/everybody_codes/kingdom_of_algorithmia/Q7/P2_T.txt";
    input = std::fs::read_to_string(path).unwrap();
    track = get_track(input);
    println!(
        "  │  ├─ Part 2: {}",
        rank_device_plans(&devices, &track, 10)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/Q7/P3.txt";
    input = std::fs::read_to_string(path).unwrap();
    devices = get_devices(input);

    path = "input/everybody_codes/kingdom_of_algorithmia/Q7/P3_T.txt";
    input = std::fs::read_to_string(path).unwrap();
    track = get_track(input);

    let plan = devices.values().next().unwrap();
    let knight_essence = run_laps(&track, plan, 2024, usize::MAX);

    let mut plans = Vec::new();
    combine(
        HashMap::from([('+', 5), ('-', 3), ('=', 3)]),
        Vec::new(),
        &mut plans,
    );

    println!(
        "  │  └─ Part 3: {}",
        plans
            .iter()
            .map(|p| run_laps(&track, p, 2024, knight_essence))
            .filter(|&e| e > knight_essence)
            .count()
    );
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

    let mut last: (usize, usize) = (0, 0);
    let mut curr: (usize, usize) = (0, 0);
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    loop {
        let mut next = None;
        for &d in &directions {
            let next_candidate: (i32, i32) = (curr.0 as i32 + d.0, curr.1 as i32 + d.1);
            if next_candidate.0 < 0 || next_candidate.1 < 0 {
                continue;
            }

            let next_inside = (next_candidate.0 as usize, next_candidate.1 as usize);
            if next_inside.0 >= raw_track.len()
                || next_inside.1 >= raw_track[next_inside.0].len()
                || raw_track[next_inside.0][next_inside.1] == ' '
                || next_inside == last
            {
                continue;
            }

            next = Some(next_inside);
            break;
        }

        match next {
            Some(n) => {
                track.push(raw_track[n.0][n.1]);
                last = curr;
                curr = n;

                if raw_track[curr.0][curr.1] == 'S' {
                    break;
                }
            }
            None => break,
        }
    }

    track
}

fn rank_device_plans(devices: &HashMap<String, Vec<char>>, track: &[char], laps: usize) -> String {
    let mut ranking = devices
        .iter()
        .map(|(id, plan)| (id.clone(), run_laps(track, plan, laps, usize::MAX)))
        .collect::<Vec<(String, usize)>>();
    ranking.sort_by(|p, n| n.1.cmp(&p.1));
    ranking.iter().map(|(id, _)| id.clone()).collect::<String>()
}

fn run_laps(track: &[char], plan: &[char], laps: usize, stop_at: usize) -> usize {
    let mut power = 10;
    let mut sum = 0;
    let mut pos = 0;

    for _ in 0..laps {
        sum += run_track(track, plan, &mut power, &mut pos);
        if sum >= stop_at {
            break;
        }
    }

    sum
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

fn combine(to_use: HashMap<char, usize>, current: Vec<char>, plans: &mut Vec<Vec<char>>) {
    if to_use.is_empty() {
        plans.push(current.clone());
    }

    for &action in to_use.keys() {
        let mut updated_to_use = to_use.clone();
        let entry = updated_to_use.get_mut(&action).unwrap();
        if *entry == 1 {
            updated_to_use.remove(&action);
        } else {
            *entry -= 1;
        }

        let mut updated_current = current.clone();
        updated_current.push(action);
        combine(updated_to_use, updated_current, plans);
    }
}
