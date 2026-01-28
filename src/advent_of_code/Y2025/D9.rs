use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::ops::Bound::Excluded;

pub fn run() {
    println!("  ├─ Day 9 - Movie Theater");

    let path = "input/advent_of_code/Y2025/D9.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let red_tiles = input.lines().map(Point::from).collect::<Vec<Point>>();

    let mut x_edges = BTreeMap::new();
    let mut y_edges = BTreeMap::new();

    let mut add_edge = |t1: &Point, t2: &Point| {
        let x_1 = t1.x;
        let x_2 = t2.x;
        let y_1 = t1.y;
        let y_2 = t2.y;

        match x_1 != x_2 {
            true => y_edges
                .entry(t1.y)
                .or_insert(Vec::new())
                .push((x_1.min(x_2), x_1.max(x_2))),
            false => x_edges
                .entry(t1.x)
                .or_insert(Vec::new())
                .push((y_1.min(y_2), y_1.max(y_2))),
        }
    };
    red_tiles
        .windows(2)
        .for_each(|pair| add_edge(&pair[0], &pair[1]));
    add_edge(&red_tiles[red_tiles.len() - 1], &red_tiles[0]);

    let rectangle_contains_edge = |(t1, t2): (&Point, &Point),
                                   edges_at_an_x: &BTreeMap<u64, Vec<(u64, u64)>>,
                                   edges_at_an_y: &BTreeMap<u64, Vec<(u64, u64)>>|
     -> bool {
        let (x_min, x_max) = (t1.x.min(t2.x), t1.x.max(t2.x));
        let (y_min, y_max) = (t1.y.min(t2.y), t1.y.max(t2.y));

        if x_min + 1 >= x_max || y_min + 1 >= y_max {
            return false;
        }

        let do_segments_overlap =
            |s1: u64, e1: u64, s2: u64, e2: u64| -> bool { s1.max(s2) < e1.min(e2) };

        let check_if_edges_cut_rectangle =
            |value_start: u64,
             value_end: u64,
             perpendicular_edges_at_value: &BTreeMap<u64, Vec<(u64, u64)>>,
             rectangle_start: u64,
             rectangle_end: u64|
             -> bool {
                for (_, edges) in perpendicular_edges_at_value
                    .range((Excluded(&value_start), Excluded(&value_end)))
                {
                    for &(start, end) in edges {
                        if do_segments_overlap(start, end, rectangle_start, rectangle_end) {
                            return true;
                        }
                    }
                }
                false
            };

        check_if_edges_cut_rectangle(y_min, y_max, &edges_at_an_y, x_min, x_max)
            || check_if_edges_cut_rectangle(x_min, x_max, &edges_at_an_x, y_min, y_max)
    };

    let point_is_inside_polygon =
        |point: &Point, edges_at_an_x: &BTreeMap<u64, Vec<(u64, u64)>>| -> bool {
            let mut intersect_edge_count = 0;
            for (_, y_edges) in edges_at_an_x.range(0..=point.x) {
                for &(start, end) in y_edges {
                    if start <= point.y && point.y <= end {
                        intersect_edge_count += 1;
                        break;
                    }
                }
            }

            intersect_edge_count % 2 == 1
        };

    let mut max_area = 0;
    let mut max_bounded_area = 0;
    for (i, t1) in red_tiles.iter().enumerate() {
        for t2 in red_tiles.iter().skip(i + 1) {
            let area = t1.area(t2);
            max_area = area.max(max_area);

            if area <= max_bounded_area
                || rectangle_contains_edge((t1, t2), &x_edges, &y_edges)
                || !point_is_inside_polygon(t1, &x_edges)
            {
                continue;
            }

            max_bounded_area = area;
            println!("{t1} - {t2} = {area}");
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
