use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("  ├─ Quest 2 - The Runes of Power");

    let (mut words, mut sections) =
        read_file("input/everybody_codes/kingdom_of_algorithmia/Q2/P1.txt".to_string());

    let word_count = get_word_count_in_inscription(words, sections[0].to_string());

    println!("  │  ├─ Part 1: {}", word_count);

    (words, sections) =
        read_file("input/everybody_codes/kingdom_of_algorithmia/Q2/P2.txt".to_string());

    let mut matrix = sections.iter().map(|s| s.chars().collect()).collect();
    let mut symbol_count =
        get_in_matrix_inscription(words, matrix, false, vec![search_forward, search_backwards]);

    println!("  │  ├─ Part 2: {}", symbol_count);

    (words, sections) =
        read_file("input/everybody_codes/kingdom_of_algorithmia/Q2/P3.txt".to_string());

    matrix = sections
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let wrapped_downwards_search =
        |a: &str, b: &[Vec<char>], c: (usize, usize), _: bool| search_downwards(a, b, c);
    let wrapped_upwards_search =
        |a: &str, b: &[Vec<char>], c: (usize, usize), _: bool| search_upwards(a, b, c);

    symbol_count = get_in_matrix_inscription(
        words,
        matrix,
        true,
        vec![
            search_forward,
            search_backwards,
            wrapped_downwards_search,
            wrapped_upwards_search,
        ],
    );
    println!("  │  └─ Part 3: {}", symbol_count);
}

fn read_file(path: String) -> (HashMap<char, Vec<String>>, Vec<String>) {
    let input = std::fs::read_to_string(path).unwrap();
    let sections = input
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut words: HashMap<char, Vec<String>> = HashMap::new();
    sections[0].split(":").collect::<Vec<&str>>()[1]
        .split(",")
        .for_each(|w| {
            let first = w.chars().next().unwrap();
            words.entry(first).or_default().push(w.to_string());
        });

    (words, sections[2..].to_vec())
}

fn get_word_count_in_inscription(words: HashMap<char, Vec<String>>, inscription: String) -> u32 {
    let mut count = 0;

    for part in inscription.split(" ") {
        let chars = part.chars().collect::<Vec<char>>();

        for i in 0..chars.len() {
            if let Some(start_with) = words.get(&chars[i]) {
                let partition = &part[i..part.len()];

                for w in start_with {
                    if partition.starts_with(w) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

type SearchMethod = fn(&str, &[Vec<char>], (usize, usize), bool) -> Option<HashSet<(usize, usize)>>;

fn get_in_matrix_inscription(
    words: HashMap<char, Vec<String>>,
    inscription: Vec<Vec<char>>,
    connected_edges: bool,
    search_methods: Vec<SearchMethod>,
) -> usize {
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..inscription.len() {
        for j in 0..inscription[i].len() {
            let character = inscription[i][j];
            if !words.contains_key(&character) {
                continue;
            }

            let starts_with = words.get(&character).unwrap();
            starts_with.iter().for_each(|w| {
                let mut found: HashSet<(usize, usize)> = HashSet::new();

                for method in search_methods.iter() {
                    if let Some(positions) = method(w, &inscription, (i, j), connected_edges) {
                        found.extend(positions);
                    }
                }

                found.iter().for_each(|p| {
                    symbols.insert(*p);
                })
            })
        }
    }

    symbols.len()
}

fn search_forward(
    word: &str,
    matrix: &[Vec<char>],
    pos: (usize, usize),
    connected_edges: bool,
) -> Option<HashSet<(usize, usize)>> {
    let mut found: HashSet<(usize, usize)> = HashSet::new();
    found.insert(pos);

    let x = pos.0;

    for i in 1..word.len() {
        let y = if connected_edges {
            (pos.1 + i) % matrix[x].len()
        } else {
            pos.1 + i
        };
        let character = word.chars().nth(i).unwrap();

        if y >= matrix[x].len() || matrix[x][y] != character {
            return None;
        }
        found.insert((x, y));
    }

    Some(found)
}

fn search_backwards(
    word: &str,
    matrix: &[Vec<char>],
    pos: (usize, usize),
    connected_edges: bool,
) -> Option<HashSet<(usize, usize)>> {
    let mut found: HashSet<(usize, usize)> = HashSet::new();
    found.insert(pos);

    let x = pos.0;
    let len = matrix[x].len();

    for i in 1..word.len() {
        let y: i32 = if connected_edges {
            (pos.1 as i32 - i as i32 + len as i32).rem_euclid(len as i32)
        } else {
            pos.1 as i32 - i as i32
        };
        let character = word.chars().nth(i).unwrap();

        if y < 0 || matrix[x][y as usize] != character {
            return None;
        }
        found.insert((x, y as usize));
    }

    Some(found)
}

fn search_upwards(
    word: &str,
    matrix: &[Vec<char>],
    pos: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let mut found: HashSet<(usize, usize)> = HashSet::new();
    found.insert(pos);

    let y = pos.1;

    for i in 1..word.len() {
        let x: i32 = pos.0 as i32 - i as i32;
        let character = word.chars().nth(i).unwrap();

        if x < 0 || matrix[x as usize][y] != character {
            return None;
        }
        found.insert((x as usize, y));
    }

    Some(found)
}

fn search_downwards(
    word: &str,
    matrix: &[Vec<char>],
    pos: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let mut found: HashSet<(usize, usize)> = HashSet::new();
    found.insert(pos);

    let y = pos.1;

    for i in 1..word.len() {
        let x = pos.0 + i;
        let character = word.chars().nth(i).unwrap();

        if x >= matrix.len() || matrix[x][y] != character {
            return None;
        }
        found.insert((x, y));
    }

    Some(found)
}
