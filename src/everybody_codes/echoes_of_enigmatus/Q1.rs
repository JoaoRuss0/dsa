use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Quest 1 - EniCode");

    let mut path = "input/everybody_codes/echoes_of_enigmatus/Q1/P1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  ├─ Part 1: {}",
        input
            .lines()
            .map(get_parameters)
            .map(|p| get_last_remainders(p.X, &eni(p.A, p.X, p.M).1, p.X)
                + get_last_remainders(p.Y, &eni(p.B, p.Y, p.M).1, p.X)
                + get_last_remainders(p.Z, &eni(p.C, p.Z, p.M).1, p.X))
            .max()
            .unwrap()
    );

    path = "input/everybody_codes/echoes_of_enigmatus/Q1/P2.txt";
    input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  ├─ Part 2: {}",
        input
            .lines()
            .map(get_parameters)
            .map(|p| {
                get_last_remainders(5, &eni(p.A, p.X, p.M).1, p.X)
                    + get_last_remainders(5, &eni(p.B, p.Y, p.M).1, p.X)
                    + get_last_remainders(5, &eni(p.C, p.Z, p.M).1, p.X)
            })
            .max()
            .unwrap()
    );

    path = "input/everybody_codes/echoes_of_enigmatus/Q1/P3.txt";
    input = std::fs::read_to_string(path).unwrap();

    println!(
        "  │  └─ Part 3: {}",
        input
            .lines()
            .map(get_parameters)
            .map(|p| {
                let mut result = 0;

                let (mut loop_start, mut remainders) = eni(p.A, p.X, p.M);
                result += sum_remainders(&remainders, p.X, loop_start);
                (loop_start, remainders) = eni(p.B, p.Y, p.M);
                result += sum_remainders(&remainders, p.Y, loop_start);
                (loop_start, remainders) = eni(p.C, p.Z, p.M);
                result += sum_remainders(&remainders, p.Z, loop_start);

                result
            })
            .max()
            .unwrap()
    );
}

fn eni(n: usize, exp: usize, modulo: usize) -> (Option<usize>, Vec<u64>) {
    let mut score = 1;

    let mut i = exp as u32;
    let mut remainders = Vec::new();
    let mut seen = HashSet::new();

    while i > 0 {
        let remainder = (score * n as u64).rem_euclid(modulo as u64);
        if seen.contains(&remainder) {
            return (remainders.iter().position(|&x| x == remainder), remainders);
        }

        seen.insert(remainder);
        remainders.push(remainder);
        score = remainder;
        i -= 1;
    }

    (None, remainders)
}

fn sum_remainders(remainders: &[u64], iterations: usize, loops_pos: Option<usize>) -> u64 {
    let loop_start = loops_pos.unwrap_or(0);
    let prefix = remainders[0..loop_start].iter().sum::<u64>();

    let mut leftover = iterations - loop_start;

    let size = remainders.len() - loop_start;
    let loops = (iterations - loop_start) / size;
    let main = remainders[loop_start..].iter().sum::<u64>() * loops as u64;

    leftover -= loops * size;

    let suffix = remainders[loop_start..loop_start + leftover]
        .iter()
        .sum::<u64>();
    prefix + main + suffix
}

fn get_last_remainders(cap: usize, remainders: &[u64], iterations: usize) -> u64 {
    let mut digit = 0;
    let mut i = get_start_pos(cap, iterations, remainders.len());

    let mut result = 0;
    let mut j = 0;

    while j < cap {
        let remainder = remainders[i % remainders.len()];
        if remainder == 0 {
            break;
        }
        result += remainder * 10_u64.pow(digit);
        digit += count_digits(remainder);

        j += 1;
        i += 1;
    }

    result
}

fn get_start_pos(bound: usize, exp: usize, loop_len: usize) -> usize {
    let stopped_at = exp.rem_euclid(loop_len);
    if stopped_at > bound {
        return stopped_at - bound;
    }

    let remaining = bound - stopped_at;
    loop_len - (remaining.rem_euclid(loop_len))
}

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

struct Parameters {
    A: usize,
    B: usize,
    C: usize,
    X: usize,
    Y: usize,
    Z: usize,
    M: usize,
}

fn get_parameters(l: &str) -> Parameters {
    let assignments = l.split(" ").collect::<Vec<&str>>();
    let values = assignments
        .iter()
        .map(|a| a.split_once("=").unwrap().1.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Parameters {
        A: values[0],
        B: values[1],
        C: values[2],
        X: values[3],
        Y: values[4],
        Z: values[5],
        M: values[6],
    }
}
