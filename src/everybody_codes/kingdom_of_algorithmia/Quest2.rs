pub fn run() {
    println!("  ├─ Quest 2 - The Runes of Power");

    let mut path = "input/everybody_codes/kingdom_of_algorithmia/quest2/part1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    let sections = input.split("\n").collect::<Vec<&str>>();
    let words = sections[0].split(":").collect::<Vec<&str>>()[1]
        .split(",")
        .collect::<Vec<&str>>();

    println!(
        "  │  ├─ Part 1: {}",
        get_words_in_inscription(words, sections[2])
    );

    path = "input/everybody_codes/kingdom_of_algorithmia/quest2/part2.txt";
    input = std::fs::read_to_string(path).unwrap();

    //println!("  │  ├─ Part 2: {}", );

    //path = "input/everybody_codes/kingdom_of_algorithmia/quest2/part3.txt";
    //input = std::fs::read_to_string(path).unwrap();
    //println!("  │  └─ Part 3: {}", );
}

fn get_words_in_inscription(words: Vec<&str>, inscription: &str) -> usize {
    let mut count = 0;

    for part in inscription.split(" ") {
        let mut i = 0;
        while i < words.len() {
            if part.contains(words[i]) {
                count += 1;
            }
            i += 1;
        }
    }

    count
}
