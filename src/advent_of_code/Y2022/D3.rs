use std::collections::HashSet;
use std::str::Chars;
use clap::builder::TypedValueParser;

pub fn run() {
    println!("  ├─ Day 3 - Rucksack Reorganization");

    let path = "input/advent_of_code/Y2022/D3.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let priorities = input.lines().map(|l| {
        let mut sum = 0;
        let mut chars: Vec<char> = l.chars().collect();

        let mut p1 = 0;
        let mut p2 = l.len()/2;

        let mut seen1: u64 = 0;
        let mut seen2: u64 = 0;

        let track = |c: char, seen: &mut u64| {
          *seen |= 1 << match c {
              'a'..='z' => c as u64 - 'a' as u64,
              _         => c as u64 - 'A' as u64 + 26
          }
        };

        while p1 < l.len()/2 {
            track(chars[p1], &mut seen1);
            track(chars[p2], &mut seen2);

            p1 += 1;
            p2 += 1;
        }

        let similar = seen1 & seen2;
        let mut i = 0;
        while i < 52 {

            let mask = (1 << i);
            let applied = similar & mask;
            if applied == mask { sum += i + 1; }

            i+=1;
        }

        sum
    }).sum::<usize>();

    println!("  │  ├─ Part 1: {}", priorities);
    //println!("  │  └─ Part 2: {}", );
}
