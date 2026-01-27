use std::collections::BTreeMap;
use std::fmt::Formatter;

pub fn run() {
    println!("  ├─ Day 9 - Movie Theater");

    let path = "input/advent_of_code/Y2025/D9.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let red_tiles = input.lines().map(Point::from).collect::<Vec<Point>>();

    let mut x_edges = BTreeMap::new();
    let mut y_edges = BTreeMap::new();
    for pair in red_tiles.windows(2) {
        let x_1 = pair[0].x;
        let x_2 = pair[1].x;
        let y_1 = pair[0].y;
        let y_2 = pair[1].y;

        match x_1 != x_2 {
            true => y_edges
                .entry(pair[0].y)
                .or_insert(Vec::new())
                .push((x_1.min(x_2), x_1.max(x_2))),
            false => x_edges
                .entry(pair[0].x)
                .or_insert(Vec::new())
                .push((y_1.min(y_2), y_1.max(y_2))),
        }
    }

    let is_cut_by_any_edge =
        |edge: (u64, u64), at: u64, opposite_edges: &BTreeMap<u64, Vec<(u64, u64)>>| -> bool {
            let mut cut = false;

            'outer: for i in edge.0..=edge.1 {
                if let Some(edges) = opposite_edges.get(&i) {
                    for &(s, e) in edges {
                        match i == edge.0 || i == edge.1 {
                            true => {
                                if s < at && at < e {
                                    cut = true;
                                    break 'outer;
                                }
                            }
                            false => {
                                if s <= at && at <= e {
                                    cut = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }

            cut
        };

    let mut max_area = 0;
    let mut max_bounded_area = 0;
    for (i, t1) in red_tiles.iter().enumerate() {
        for t2 in red_tiles.iter().skip(i + 1) {
            let area = t1.area(t2);
            max_area = area.max(max_area);

            let x_edge = (t1.x.min(t2.x), t1.x.max(t2.x));
            let y_edge = (t1.y.min(t2.y), t1.y.max(t2.y));

            if is_cut_by_any_edge(x_edge, t1.y, &x_edges)
                || is_cut_by_any_edge(x_edge, t2.y, &x_edges)
                || is_cut_by_any_edge(y_edge, t1.x, &y_edges)
                || is_cut_by_any_edge(y_edge, t2.x, &y_edges)
            {
                continue;
            }
            max_bounded_area = max_bounded_area.max(area);
        }
    }

    println!("  │  ├─ Part 1: {}", max_area);
    println!("  │  └─ Part 2: {}", max_bounded_area);
}

struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn from(string: &str) -> Self {
        let split = string.split_once(',').unwrap();
        Self {
            x: split.0.parse::<u64>().unwrap(),
            y: split.1.parse::<u64>().unwrap(),
        }
    }

    fn area(&self, other: &Self) -> u64 {
        ((self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1))
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
