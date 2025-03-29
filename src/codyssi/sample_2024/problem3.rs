use std::ptr::read;

pub fn run() {

    println!("Problem 3");

    let input = std::fs::read_to_string("input/codyssi/sample_2024/problem3.txt")
        .unwrap();

    let base_sum : usize = input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(_, base)| base.parse::<usize>().unwrap())
        .sum();

    println!("- Part 1: {}", base_sum);

    let sum = input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(reading, base) |i64::from_str_radix(reading, base.parse::<u32>().unwrap()).unwrap())
        .sum::<i64>();

    println!("- Part 2: {}", sum);

    let base65 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#";

    let convert_to_base_65 = |sum : i64| -> String {

        let mut n_65 : String = String::new();
        let mut n = sum;

        while n > 0 {
            n_65.insert(0, base65.chars().nth((n % 65) as usize).unwrap());
            n/=65;
        }
        n_65
    };

    println!("- Part 3: {}", convert_to_base_65(sum));
}