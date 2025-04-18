pub fn run() {
    println!("  ├─ Problem 4 - Aeolian Transmissions");

    let input = std::fs::read_to_string("input/codyssi/journey_to_atlantis/problem4.txt").unwrap();

    let get_char_size = |c: char| -> u64 {
        match c {
            'A'..='Z' => ((c as u8 - b'A') + 1) as u64,
            '0'..='9' => (c as u8 - b'0') as u64,
            _ => panic!("Character not supported"),
        }
    };

    let get_message_size =
        |message: String| -> u64 { message.chars().map(get_char_size).sum::<u64>() };

    println!(
        "  │  ├─ Part 1: {}",
        input
            .lines()
            .map(|l| get_message_size(l.to_string()))
            .sum::<u64>()
    );

    let compress = |message: &str| -> String {
        let length = message.len();
        let kept = length / 10;

        let chars = message.chars();
        let mut compressed = String::new();

        (0..kept).for_each(|i| compressed.push(chars.clone().nth(i).unwrap()));
        compressed.push_str(&(length - kept * 2).to_string());
        (length - kept..length).for_each(|i| compressed.push(chars.clone().nth(i).unwrap()));

        compressed
    };

    println!(
        "  │  ├─ Part 2: {}",
        input
            .lines()
            .map(compress)
            .map(get_message_size)
            .sum::<u64>()
    );

    let compress_losslessly = |message: &str| -> String {
        let mut compressed = String::new();

        let mut c = message.chars().next().unwrap();
        let mut count = 1;
        let mut i = 1;
        while i < message.len() {
            let next = message.chars().nth(i).unwrap();
            if next != c {
                compressed.push_str(&count.to_string());
                compressed.push(c);
                c = next;
                count = 0;
            }
            count += 1;
            i += 1;
        }
        compressed.push_str(&count.to_string());
        compressed.push(c);
        compressed
    };

    println!(
        "  │  └─ Part 3: {}",
        input
            .lines()
            .map(compress_losslessly)
            .map(get_message_size)
            .sum::<u64>()
    );
}
