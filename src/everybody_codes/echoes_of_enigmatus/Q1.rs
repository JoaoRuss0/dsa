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
            .map(|p| { eni(p.A, p.X, p.M) + eni(p.B, p.Y, p.M) + eni(p.C, p.Z, p.M) })
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
                eni_with_cap(p.A, p.X, p.M, Some(5))
                    + eni_with_cap(p.B, p.Y, p.M, Some(5))
                    + eni_with_cap(p.C, p.Z, p.M, Some(5))
            })
            .max()
            .unwrap()
    );

    //path = "input/everybody_codes/echoes_of_enigmatus/Q1/P3.txt";
    //input = std::fs::read_to_string(path).unwrap();
    //println!("  │  └─ Part 3: {}", );
}

fn eni(n: usize, exp: usize, modulo: usize) -> u64 {
    eni_with_cap(n, exp, modulo, None)
}

fn eni_with_cap(n: usize, exp: usize, modulo: usize, cap: Option<usize>) -> u64 {
    let mut score = 1;

    let mut i = exp as u32;
    let mut remainders = Vec::new();
    let mut seen = HashSet::new();
    //let mut loop_idx = 0;

    while i > 0 {
        let remainder = (score * n as u64).rem_euclid(modulo as u64);
        if seen.contains(&remainder) {
            //loop_idx = scores.iter().position(|&x| x == remainder).unwrap();
            break;
        }

        seen.insert(remainder);
        remainders.push(remainder);
        score = remainder;
        i -= 1;
    }

    let bound = cap.unwrap_or(exp);
    let mut digit = 0;
    let mut i = get_start_pos(bound, exp, seen.len());

    let mut result = 0;
    let mut j = 0;

    while j < bound {
        let remainder = remainders[i % seen.len()];
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
