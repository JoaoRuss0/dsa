pub fn run() {
    println!("  ├─ Day 2 - Rock Paper Scissors");

    let path = "input/advent_of_code/Y2022/D2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    println!("  │  ├─ Part 1: {}", input.lines()
        .map(|l| {
            let (player2, me) = l.split_once(' ')
                .map(|(a, b)| (Shape::from(a), Shape::from(b)))
                .unwrap();
            me.score() + me.outcome(&player2).score()
        })
        .sum::<u32>());

    println!("  │  └─ Part 2: {}", input.lines()
        .map(|l| {
            let (play, outcome) = l.split_once(' ')
                .map(|(a, b)| (Shape::from(a), Outcome::from(b)))
                .unwrap();

            let play_score = match outcome {
                Outcome::Won => play.loses_to().score(),
                Outcome::Lost => play.beats().score(),
                Outcome::Tied => play.score()
            };

            play_score + outcome.score()
        })
        .sum::<u32>()
    );
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}


impl From<&str> for Shape {
    fn from(value: &str) -> Self {

        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!(format!("Unknown value: {}", value))
        }
    }
}

enum Outcome {
    Won,
    Lost,
    Tied
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lost,
            "Y" => Outcome::Tied,
            "Z" => Outcome::Won,
            _ => panic!(format!("Unknown value: {}", value))
        }
    }
}

impl Outcome {

    fn score(&self) -> u32 {
        match self {
            Outcome::Won => 6,
            Outcome::Lost => 0,
            Outcome::Tied => 3
        }
    }
}

impl Shape {

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome(&self, other: &Self) -> Outcome {
        if self == other {
            Outcome::Tied
        } else if self.beats() == *other {
            Outcome::Won
        } else {
            Outcome::Lost
        }
    }

    fn beats(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}