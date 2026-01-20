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

    let a = ranges
        .iter()
        .map(Range::compute_invalid)
        .flatten()
        .collect::<Vec<usize>>();

    println!("  │  ├─ Part 1: {}", a.iter().sum::<usize>());
    //println!("  │  └─ Part 2: {}", );
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

    fn compute_invalid(&self) -> Vec<usize> {
        let mut invalid = Vec::new();

        for i in (self.min..=self.max) {
            let mut half_1 = i;
            let mut half_2: usize = 0;

            let digit_count = (i.checked_ilog10().unwrap_or(0) + 1) as usize;
            if digit_count % 2 == 1 {
                continue;
            }

            for i in 0..digit_count / 2 {
                let digit = half_1 % 10;
                let decimal_place = (10_usize.pow(i as u32)).max(1);
                half_2 += digit * decimal_place;
                half_1 /= 10;
            }

            if half_1 == half_2 {
                invalid.push(i);
            }
        }

        invalid
    }
}
