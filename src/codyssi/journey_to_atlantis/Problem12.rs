pub fn run() {
    println!("  ├─ Problem 12 - Challenging the Whirlpool");

    let path = "input/codyssi/journey_to_atlantis/problem12.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let grid: Vec<Vec<i64>> = sections[0]
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let instructions = sections[1]
        .lines()
        .map(Instruction::of)
        .collect::<Vec<Instruction>>();

    let flow_control = sections[2]
        .lines()
        .map(FlowControl::of)
        .collect::<Vec<FlowControl>>();

    let mut applied = grid.clone();
    apply_instructions(&mut applied, &instructions);

    let mut largest = 0;
    for i in 0..applied.len() {
        let col: i64 = applied[i].iter().sum();
        let row: i64 = applied.iter().map(|r| r[i]).sum();
        largest = row.max(col.max(largest));
    }

    println!("  │  ├─ Part 1: {}", largest);

    println!("  │  ├─ Part 2: {}",);
    //println!("  │  └─ Part 3: {}", );
}

fn apply_instructions(grid: &mut [Vec<i64>], instructions: &Vec<Instruction>) {
    for instruction in instructions {
        match instruction.direction {
            Direction::Col { col } => match instruction.operation {
                Operation::Shift => {
                    apply_shift_operation(instruction, grid);
                }
                _ => {
                    for i in 0..grid.len() {
                        apply_non_shift_operation(instruction, i, col, grid);
                    }
                }
            },
            Direction::Row { row } => match instruction.operation {
                Operation::Shift => apply_shift_operation(instruction, grid),
                _ => {
                    for i in 0..grid[row].len() {
                        apply_non_shift_operation(instruction, row, i, grid);
                    }
                }
            },
            Direction::All => match instruction.operation {
                Operation::Shift => panic!("Not possible"),
                _ => {
                    for i in 0..grid.len() {
                        for j in 0..grid[i].len() {
                            apply_non_shift_operation(&instruction, i, j, grid)
                        }
                    }
                }
            },
        }
    }
}

fn apply_shift_operation(instruction: &Instruction, grid: &mut [Vec<i64>]) {
    match instruction.direction {
        Direction::Col { col } => {
            let amount = (instruction.amount % grid.len() as i64) as usize;
            let mut temp = Vec::new();

            for i in 0..grid.len() {
                let row = (grid.len() - amount + i) % grid.len();
                let value = grid[row][col];
                temp.push(value);
            }

            for i in 0..grid.len() {
                grid[i][col] = temp[i];
            }
        }
        Direction::Row { row } => {
            let amount = (instruction.amount % grid[row].len() as i64) as usize;
            grid[row].rotate_right(amount);
        }
        Direction::All => {
            panic!("Not possible");
        }
    }
}

fn apply_non_shift_operation(instruction: &Instruction, i: usize, j: usize, grid: &mut [Vec<i64>]) {
    match instruction.operation {
        Operation::Add => grid[i][j] += instruction.amount,
        Operation::Sub => grid[i][j] -= instruction.amount,
        Operation::Multiply => grid[i][j] *= instruction.amount,
        _ => panic!("Unknown operation"),
    }

    if grid[i][j] > 1073741824 || grid[i][j] < 0 {
        grid[i][j] = grid[i][j].rem_euclid(1073741824);
    }
}

#[derive(Debug)]
enum Direction {
    Col { col: usize },
    Row { row: usize },
    All,
}

impl Direction {
    pub fn of(params: Vec<&str>) -> Self {
        match params[0] {
            "COL" => Direction::Col {
                col: params[1].parse::<usize>().unwrap() - 1,
            },
            "ROW" => Direction::Row {
                row: params[1].parse::<usize>().unwrap() - 1,
            },
            _ => Direction::All,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Multiply,
    Shift,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    amount: i64,
    direction: Direction,
}

impl Instruction {
    pub fn of(string: &str) -> Self {
        let split = string.split(" ").collect::<Vec<&str>>();
        let operation = split[0];
        let params = split[1..].to_vec();

        let amount;
        let direction;
        match operation {
            "SHIFT" => {
                amount = params[3].parse::<i64>().unwrap();
                direction = Direction::of(params[0..].to_vec());
            }
            _ => {
                amount = params[0].parse::<i64>().unwrap();
                direction = Direction::of(params[1..].to_vec());
            }
        }

        match operation {
            "ADD" => Instruction {
                amount,
                direction,
                operation: Operation::Add,
            },
            "SUB" => Instruction {
                amount,
                direction,
                operation: Operation::Sub,
            },
            "MULTIPLY" => Instruction {
                amount,
                direction,
                operation: Operation::Multiply,
            },
            "SHIFT" => Instruction {
                amount,
                direction,
                operation: Operation::Shift,
            },
            _ => panic!("Unknown operation: {}", operation),
        }
    }
}

enum FlowControl {
    Take,
    Cycle,
    Act,
}

impl FlowControl {
    pub fn of(string: &str) -> Self {
        match string {
            "TAKE" => FlowControl::Take,
            "CYCLE" => FlowControl::Cycle,
            "ACT" => FlowControl::Act,
            _ => panic!("Unknown flow control: {}", string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let instruction = Instruction {
            operation: Operation::Shift,
            amount: 2,
            direction: Direction::Col { col: 1 },
        };
        apply_shift_operation(&instruction, &mut grid);
        assert_eq!(vec![vec![1, 5, 3], vec![4, 8, 6], vec![7, 2, 9]], grid);
    }

    #[test]
    fn test2() {
        let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let instruction = Instruction {
            operation: Operation::Shift,
            amount: 206,
            direction: Direction::Row { row: 2 },
        };
        apply_shift_operation(&instruction, &mut grid);
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![8, 9, 7]], grid);
    }

    #[test]
    fn test3() {
        let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let instruction = Instruction {
            operation: Operation::Shift,
            amount: 23,
            direction: Direction::Row { row: 1 },
        };
        apply_shift_operation(&instruction, &mut grid);
        assert_eq!(vec![vec![1, 2, 3], vec![5, 6, 4], vec![7, 8, 9]], grid);
    }

    #[test]
    fn test4() {
        let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let instruction = Instruction {
            operation: Operation::Add,
            amount: 450,
            direction: Direction::Row { row: 2 },
        };
        apply_instructions(&mut grid, &vec![instruction]);
        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![457, 458, 459]],
            grid
        );
    }

    #[test]
    fn test5() {
        let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let instruction = Instruction {
            operation: Operation::Sub,
            amount: 230,
            direction: Direction::Col { col: 0 },
        };
        apply_instructions(&mut grid, &vec![instruction]);
        assert_eq!(
            vec![vec![-229, 2, 3], vec![-226, 5, 6], vec![-223, 8, 9]],
            grid
        );
    }

    #[test]
    fn test6() {
        let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let instruction = Instruction {
            operation: Operation::Multiply,
            amount: 43,
            direction: Direction::Col { col: 0 },
        };
        apply_instructions(&mut grid, &vec![instruction]);
        assert_eq!(vec![vec![43, 2, 3], vec![172, 5, 6], vec![301, 8, 9]], grid);
    }

    #[test]
    fn test7() {
        let mut grid = vec![
            vec![222, 267, 922, 632, 944],
            vec![110, 33, 503, 758, 129],
            vec![742, 697, 425, 362, 568],
            vec![833, 408, 425, 349, 631],
            vec![874, 671, 202, 430, 602],
        ];

        let ins_1 = Instruction {
            operation: Operation::Shift,
            amount: 1,
            direction: Direction::Col { col: 1 },
        };
        apply_instructions(&mut grid, &vec![ins_1]);
        assert_eq!(
            vec![
                vec![222, 671, 922, 632, 944],
                vec![110, 267, 503, 758, 129],
                vec![742, 33, 425, 362, 568],
                vec![833, 697, 425, 349, 631],
                vec![874, 408, 202, 430, 602],
            ],
            grid
        );

        let ins_2 = Instruction {
            operation: Operation::Multiply,
            amount: 4,
            direction: Direction::Col { col: 4 },
        };
        apply_instructions(&mut grid, &vec![ins_2]);
        assert_eq!(
            vec![
                vec![222, 671, 922, 632, 3776],
                vec![110, 267, 503, 758, 516],
                vec![742, 33, 425, 362, 2272],
                vec![833, 697, 425, 349, 2524],
                vec![874, 408, 202, 430, 2408],
            ],
            grid
        );

        let ins_3 = Instruction {
            operation: Operation::Sub,
            amount: 28,
            direction: Direction::All,
        };
        apply_instructions(&mut grid, &vec![ins_3]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 604, 3748],
                vec![82, 239, 475, 730, 488],
                vec![714, 5, 397, 334, 2244],
                vec![805, 669, 397, 321, 2496],
                vec![846, 380, 174, 402, 2380],
            ],
            grid
        );

        let ins_4 = Instruction {
            operation: Operation::Shift,
            amount: 2,
            direction: Direction::Col { col: 3 },
        };
        apply_instructions(&mut grid, &vec![ins_4]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 321, 3748],
                vec![82, 239, 475, 402, 488],
                vec![714, 5, 397, 604, 2244],
                vec![805, 669, 397, 730, 2496],
                vec![846, 380, 174, 334, 2380],
            ],
            grid
        );

        let ins_5 = Instruction {
            operation: Operation::Multiply,
            amount: 4,
            direction: Direction::Row { row: 3 },
        };
        apply_instructions(&mut grid, &vec![ins_5]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 321, 3748],
                vec![82, 239, 475, 402, 488],
                vec![714, 5, 397, 604, 2244],
                vec![3220, 2676, 1588, 2920, 9984],
                vec![846, 380, 174, 334, 2380],
            ],
            grid
        );

        let ins_6 = Instruction {
            operation: Operation::Add,
            amount: 26,
            direction: Direction::Row { row: 2 },
        };
        apply_instructions(&mut grid, &vec![ins_6]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 321, 3748],
                vec![82, 239, 475, 402, 488],
                vec![740, 31, 423, 630, 2270],
                vec![3220, 2676, 1588, 2920, 9984],
                vec![846, 380, 174, 334, 2380],
            ],
            grid
        );

        let ins_7 = Instruction {
            operation: Operation::Shift,
            amount: 2,
            direction: Direction::Col { col: 3 },
        };
        apply_instructions(&mut grid, &vec![ins_7]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 2920, 3748],
                vec![82, 239, 475, 334, 488],
                vec![740, 31, 423, 321, 2270],
                vec![3220, 2676, 1588, 402, 9984],
                vec![846, 380, 174, 630, 2380],
            ],
            grid
        );

        let ins_8 = Instruction {
            operation: Operation::Add,
            amount: 68,
            direction: Direction::Row { row: 1 },
        };
        apply_instructions(&mut grid, &vec![ins_8]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 2920, 3748],
                vec![150, 307, 543, 402, 556],
                vec![740, 31, 423, 321, 2270],
                vec![3220, 2676, 1588, 402, 9984],
                vec![846, 380, 174, 630, 2380],
            ],
            grid
        );

        let ins_9 = Instruction {
            operation: Operation::Shift,
            amount: 8,
            direction: Direction::Row { row: 2 },
        };
        apply_instructions(&mut grid, &vec![ins_9]);
        assert_eq!(
            vec![
                vec![194, 643, 894, 2920, 3748],
                vec![150, 307, 543, 402, 556],
                vec![423, 321, 2270, 740, 31],
                vec![3220, 2676, 1588, 402, 9984],
                vec![846, 380, 174, 630, 2380],
            ],
            grid
        );
    }
}
