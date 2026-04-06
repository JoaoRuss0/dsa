use std::collections::HashSet;

pub fn run() {
    println!("  ├─ Day 12 - Christmas Tree Farm");

    let path = "input/advent_of_code/Y2025/D12.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let split = input.split("\n\n").collect::<Vec<&str>>();
    let mut shapes = split[0..split.len() - 1]
        .iter()
        .map(|&s| Shape::from(s))
        .collect::<Vec<Shape>>();

    let mut spaces = split
        .last()
        .unwrap()
        .lines()
        .map(Space::from)
        .collect::<Vec<Space>>();

    println!("  │  └─ Part 1: {}", 1);
}

struct Space {
    size: (usize, usize),
    needed: Vec<usize>,
}

impl Space {
    fn fit(&self, shapes: &Vec<Shape>) -> bool {
        //let to_fit = shapes

        false
    }
}

impl From<&str> for Space {
    fn from(value: &str) -> Self {
        let split = value.split(":").collect::<Vec<&str>>();
        let size_s = split[0].split_once("x").unwrap();
        let size = (
            size_s.0.parse::<usize>().unwrap(),
            size_s.1.parse::<usize>().unwrap(),
        );
        let needed = split[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Space { size, needed }
    }
}

struct Shape {
    variations: HashSet<Vec<Vec<bool>>>,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        let original = value[1..]
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();

        let mirror = |matrix: &Vec<Vec<bool>>| -> Vec<Vec<bool>> {
            let height = matrix.len();
            let width = matrix.first().unwrap().len();
            let mut mirrored = vec![vec![false; width]; height];

            for i in 0..height {
                for j in 0..width {
                    if !matrix[i][j] {
                        continue;
                    }
                    mirrored[i][width - 1 - j] = true;
                }
            }

            mirrored
        };

        let mirror = mirror(&original);
        let mut variations = HashSet::new();
        variations.insert(original);
        variations.insert(mirror);

        Shape { variations }
    }
}
