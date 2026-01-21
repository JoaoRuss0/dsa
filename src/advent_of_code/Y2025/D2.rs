pub fn run() {
    println!("  ├─ Day 2 - Secret Entrance");

    let path = "input/advent_of_code/Y2025/D2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let ranges = input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(Range::from)
        .collect::<Vec<Range>>();

    let invalid = ranges
        .iter()
        .map(Range::compute_invalid)
        .flatten()
        .collect::<Vec<InvalidInRange>>();

    println!(
        "  │  ├─ Part 1: {}",
        invalid
            .iter()
            .filter(|i| i.twice)
            .map(|i| i.number)
            .sum::<usize>()
    );
    println!(
        "  │  └─ Part 2: {}",
        invalid.iter().map(|i| i.number).sum::<usize>()
    );
}

struct InvalidInRange {
    twice: bool,
    number: usize,
}

struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn from(string: &str) -> Range {
        let (min_s, max_s) = string.split_once("-").unwrap();
        let min = min_s.parse::<usize>().unwrap();
        let max = max_s.parse::<usize>().unwrap();
        Range { min, max }
    }

    fn compute_invalid(&self) -> Vec<InvalidInRange> {
        let mut invalid = Vec::new();

        for n in (self.min..=self.max) {
            let digit_count = (n.checked_ilog10().unwrap_or(0) + 1) as usize;

            'chunking: for c in 2..=digit_count {
                if digit_count % c != 0 {
                    continue;
                }

                let mut digits = n;
                let mut first_chunk = None;

                for _ in 0..c {
                    let mut chunk = 0;
                    for j in 0..digit_count / c {
                        let digit = digits % 10;
                        let decimal_place = (10_usize.pow(j as u32)).max(1);
                        chunk += digit * decimal_place;
                        digits /= 10;
                    }

                    match first_chunk {
                        Some(first_chunk) => match chunk != first_chunk {
                            true => continue 'chunking,
                            false => (),
                        },
                        None => first_chunk = Some(chunk),
                    }
                }

                invalid.push(InvalidInRange {
                    twice: c == 2,
                    number: n,
                });

                break;
            }
        }

        invalid
    }
}
