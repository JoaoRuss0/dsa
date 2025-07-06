pub fn run() {
    println!("  ├─ Day 2 - Red-Nosed Reports");

    let path = "input/advent_of_code/Y2024/D2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!(
        "  │  ├─ Part 1: {}",
        reports
            .clone()
            .into_iter()
            .filter(|report| is_safe(report))
            .count()
    );
    println!(
        "  │  └─ Part 2: {}",
        reports
            .into_iter()
            .filter(|report| is_safe_dampener(report))
            .count()
    );
}

fn calc_diff(report: &[i32]) -> Vec<i32> {
    let mut diffs = Vec::new();

    for (i, level) in report.iter().enumerate().skip(1) {
        diffs.push(level - report[i - 1]);
    }

    diffs
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = None;
    let diffs = calc_diff(report);

    for &diff in diffs.iter() {
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match increasing {
            None => increasing = Some(diff > 0),
            Some(value) => {
                if value && diff < 0 || !value && diff > 0 {
                    return false;
                }
            }
        }
    }

    true
}

fn is_safe_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut new = report.to_owned();
        new.remove(i);

        if is_safe(&new) {
            return true;
        }
    }

    false
}
