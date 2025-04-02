pub fn run() {
    println!("  ├─ Problem 7 - Siren Disruption");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem7.txt").unwrap();

    let sections = input.split("\n\n").collect::<Vec<_>>();

    let tracks = sections[0]
        .lines()
        .filter(|s| !s.is_empty())
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let swaps = sections[1]
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (left, right) = s.split_once("-").unwrap();
            (
                left.parse::<usize>().unwrap() - 1,
                right.parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<Vec<_>>();

    let index = sections[2].trim().parse::<usize>().unwrap() - 1;

    let mut test = tracks.clone();
    swaps.iter().for_each(|c| test.swap(c.0, c.1));

    println!("  │  ├─ Part 1: {}", test[index]);

    let three_way_swap = |x_idx: usize, y_idx: usize, z_idx: usize, standard_2: &mut Vec<u64>| {
        let x = standard_2[x_idx];
        standard_2[x_idx] = standard_2[z_idx];
        standard_2[z_idx] = standard_2[y_idx];
        standard_2[y_idx] = x;
    };

    let mut standard_2 = tracks.clone();
    for i in 0..swaps.len() {
        let swap = swaps[i];
        let z = swaps[(i + 1) % swaps.len()].0;
        three_way_swap(swap.0, swap.1, z, &mut standard_2)
    }

    println!("  │  ├─ Part 2: {}", standard_2[index]);

    let block_len = |start: usize, len: usize, tracks: &Vec<u64>| -> usize {
        let mut length = 1;
        for i in start + 1..tracks.len() {
            if tracks[i] < tracks[i - 1] || i >= start + len {
                break;
            }
            length += 1;
        }
        length
    };

    let mut highest = tracks.clone();
    swaps.iter().for_each(|&(l, r)| {
        let biggest_x = l.max(r);
        let len_1 = tracks.len() - biggest_x;
        let len_2 = l.abs_diff(r);

        for i in 0..len_1.min(len_2) {
            highest.swap(l + i, r + i)
        }
    });

    println!("  │  └─ Part 3: {}", highest[index]);
}
