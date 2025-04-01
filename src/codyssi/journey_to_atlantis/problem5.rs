use std::collections::HashMap;
use std::hash::Hash;
use std::os::unix::raw::gid_t;

pub fn run() {
    println!("  ├─ Problem 5 - Patron Islands");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem5.txt")
        .unwrap();

    let distance = |p1: (i64, i64), p2: (i64, i64)| -> i64 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    };
    
    let points = input.lines()
        .map(|l| l.split_once(", ").unwrap())
        .map(|(x, y)| 
            ((&x[1..]).parse::<i64>().unwrap(),(&y[0..y.len()-1]).parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();

    let distances = |p: (i64, i64), points: Vec<(i64, i64)>| -> HashMap<(i64, i64), i64> {

        points.iter()
            .map(|&(x, y)| ((x, y), distance((x, y), p)))
            .collect()
    };
    
    let distances_boat = distances((0, 0), points.clone());
    
    let (&p_c, &d_c) = distances_boat.iter()
        .min_by(|&a, &b| a.1.cmp(b.1)).unwrap();
    let (&p_f, &d_f) = distances_boat.iter()
        .max_by(|&a, &b| a.1.cmp(b.1)).unwrap();
    
    println!("  │  ├─ Part 1: {}", (d_c - d_f).abs());
    
    let closest = |point: (i64, i64), points: &Vec<(i64, i64)>| -> ((i64, i64), i64) {

        let points_but_closest : Vec<(i64, i64)> = points.iter()
            .filter(|&p| !p.eq(&point))
            .map(|p| *p)
            .collect();

        let distances_closest = distances(point, points_but_closest);
        
        distances_closest.iter()
            .min_by(|(p1, d1),
                     (p2, d2)|
                d1.cmp(d2)
                    .then(p1.0.cmp(&p2.0))
                    .then(p1.1.cmp(&p2.1)))
            .map(|(&(x, y), &d)| ((x, y), d))
            .unwrap()
    };
    
    println!("  │  ├─ Part 2: {:?}", closest(p_c, &points).1);

    let mut to_visit : Vec<(i64, i64)> = points.clone();
    
    let closest_to_boat = closest((0, 0), &to_visit);
    let mut distance_sum = closest_to_boat.1;
    let mut current : (i64, i64) = closest_to_boat.0;

    while to_visit.len() > 1 {
        
        let closest = closest(current, &to_visit);
        let to_closest = distance(current, closest.0);
        distance_sum+=to_closest;
        
        let current_index = to_visit.iter()
            .position(|&p| p.eq(&current))
            .unwrap();
        to_visit.remove(current_index);
        current = closest.0;
    }
    distance_sum+=distance(current, to_visit[0]);
    
    println!("  │  └─ Part 3: {}", distance_sum);
}