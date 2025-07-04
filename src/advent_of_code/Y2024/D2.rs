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
        reports.iter().filter(|levels| is_safe(levels)).count()
    );
    //println!("  │  └─ Part 2: {}", );
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = None;
    for (i, level) in report.iter().enumerate().skip(1) {
        let diff = level - report[i - 1];
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
