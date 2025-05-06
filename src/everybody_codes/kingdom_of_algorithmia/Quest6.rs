use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("  ├─ Quest 6 - The Tree of Titans");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/quest6/part1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();
    let mut tree = build_tree(input);
    let get_node = |n: &String| -> String { n.clone() };
    println!(
        "  │  ├─ Part 1: {}",
        find_strongest_fruit_path(&tree, get_node)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/quest6/part2.txt";
    input = std::fs::read_to_string(path).unwrap();
    tree = build_tree(input);
    let get_first_char = |n: &String| -> String { n.chars().next().unwrap().to_string() };
    println!(
        "  │  ├─ Part 2: {}",
        find_strongest_fruit_path(&tree, get_first_char)
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/quest6/part3.txt";
    input = std::fs::read_to_string(path).unwrap();
    tree = build_tree(input);
    println!(
        "  │  └─ Part 3: {}",
        find_strongest_fruit_path(&tree, get_first_char)
    );
}

fn build_tree(input: String) -> HashMap<String, HashSet<String>> {
    let mut nodes = HashMap::new();

    input.lines().for_each(|line| {
        let split = line.split(":").collect::<Vec<&str>>();

        let val = split[0].to_string();
        let next = split[1]
            .split(",")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        nodes.insert(val, next.into_iter().collect());
    });

    nodes
}

fn find_strongest_fruit_path(
    tree: &HashMap<String, HashSet<String>>,
    get_path_from_node: fn(&String) -> String,
) -> String {
    let mut to_search = vec![(String::from(""), String::from("RR"))];
    let mut path_to_strongest = String::from("");

    while !to_search.is_empty() {
        let mut count: usize = 0;
        let mut next_to_search = Vec::new();

        while let Some((path, curr)) = to_search.pop() {
            if curr == "@" {
                if count == 0 {
                    path_to_strongest = path;
                    path_to_strongest.push_str(&curr);
                }
                count += 1;
                continue;
            }

            if let Some(next_nodes) = tree.get(&curr) {
                for node in next_nodes {
                    let mut path_to_next: String = path.clone();
                    path_to_next.push_str(&get_path_from_node(&curr));
                    next_to_search.push((path_to_next, node.clone()));
                }
            }
        }

        if count == 1 {
            break;
        }

        to_search = next_to_search;
    }

    path_to_strongest
}
