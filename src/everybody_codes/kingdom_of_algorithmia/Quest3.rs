use std::collections::VecDeque;

pub fn run() {
    println!("  ├─ Quest 3 - Mining Maestro");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/quest3/part1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();
    let mut matrix = build_matrix(&input);
    println!("  │  ├─ Part 1: {}", mine(&mut matrix, false));

    path = "input/everybody_codes/kingdom_of_algorithmia/quest3/part2.txt";
    input = std::fs::read_to_string(path).unwrap();
    matrix = build_matrix(&input);
    println!("  │  ├─ Part 2: {}", mine(&mut matrix, false));

    path = "input/everybody_codes/kingdom_of_algorithmia/quest3/part3.txt";
    input = std::fs::read_to_string(path).unwrap();
    matrix = build_matrix(&input);
    wrap_in_dots(&mut matrix);
    println!("  │  └─ Part 3: {}", mine(&mut matrix, true));
}

fn build_matrix(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn wrap_in_dots(matrix: &mut Vec<Vec<String>>) {
    matrix.iter_mut().for_each(|row| {
        row.insert(0, ".".to_string());
        row.push(".".to_string());
    });
    matrix.insert(0, vec![".".to_string(); matrix[0].len()]);
    matrix.push(vec![".".to_string(); matrix[0].len()]);
}

fn mine(matrix: &mut [Vec<String>], count_diagonals: bool) -> usize {
    let mut directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let diagonals = vec![(1, 1), (-1, 1), (1, -1), (-1, -1)];

    if count_diagonals {
        directions.extend(diagonals);
    }

    let mut layer = get_first_layer(matrix);
    let mut count = 0;

    while let Some((curr_x, curr_y, curr_cost)) = layer.pop_front() {
        count += curr_cost;

        for direction in directions.iter() {
            let candidate_x = curr_x as i32 + direction.0;
            let candidate_y = curr_y as i32 + direction.1;

            if candidate_x < 0 || candidate_y < 0 {
                continue;
            }

            let next_x = candidate_x as usize;
            let next_y = candidate_y as usize;
            if next_x < matrix.len()
                && next_y < matrix[next_x].len()
                && matrix[next_x][next_y] == "#"
            {
                let next_cost = curr_cost + 1;
                matrix[next_x][next_y] = next_cost.to_string();
                layer.push_back((next_x, next_y, next_cost));
            }
        }
    }

    count
}

fn get_first_layer(matrix: &[Vec<String>]) -> VecDeque<(usize, usize, usize)> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(x, l)| {
            l.iter()
                .enumerate()
                .filter(|&(_, c)| *c == ".")
                .map(|(y, _)| (x, y, 0))
                .collect::<Vec<(usize, usize, usize)>>()
        })
        .collect::<VecDeque<(usize, usize, usize)>>()
}
