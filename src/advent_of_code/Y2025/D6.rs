use crate::advent_of_code::Y2025::D6::Operation::{Add, Multiply};

pub fn run() {
    println!("  ├─ Day 6 - Trash Compactor");

    let path = "input/advent_of_code/Y2025/D6.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let problems = parse_problem(input);

    println!(
        "  │  ├─ Part 1: {}",
        problems.iter().map(Problem::solve).sum::<usize>()
    );
    //println!("  │  └─ Part 2: {}", );
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from(character: char) -> Self {
        match character {
            '*' => Multiply,
            '+' => Add,
            _ => panic!("Unknown operation"),
        }
    }
}

struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> usize {
        match self.operation {
            Add => self.numbers.iter().sum(),
            Multiply => self.numbers.iter().product(),
        }
    }
}

fn parse_problem(input: String) -> Vec<Problem> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut problem_numbers = Vec::new();

    for n in lines[0].split_whitespace() {
        let mut numbers = Vec::new();
        numbers.push(n.parse::<usize>().unwrap());
        problem_numbers.push(numbers);
    }

    for line in lines[1..lines.len() - 1].iter() {
        for (i, n) in line.split_whitespace().enumerate() {
            problem_numbers[i].push(n.parse::<usize>().unwrap());
        }
    }

    let mut problems = Vec::new();
    for (i, operation) in lines[lines.len() - 1].split_whitespace().enumerate() {
        problems.push(Problem {
            numbers: problem_numbers.get(i).unwrap().clone(),
            operation: Operation::from(operation.chars().next().unwrap()),
        })
    }

    problems
}
