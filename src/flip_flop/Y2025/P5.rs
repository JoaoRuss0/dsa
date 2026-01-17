use std::collections::HashMap;

pub fn run() {
    println!("  ├─ Puzzle 5: Strange Tunnels");

    let path = "input/flip_flop/Y2025/P5.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let sequence = input.chars().collect::<Vec<char>>();

    let mut tunnels: HashMap<char, Tunnel> = HashMap::new();

    let mut prev: Option<char> = None;
    for (i, &c) in sequence.iter().enumerate() {
        match tunnels.get_mut(&c) {
            Some(tunnel) => {
                tunnel.close(i);
            }
            None => {
                tunnels.insert(c, Tunnel::new(i));
            }
        }

        if prev.is_some() {
            let p = prev.unwrap();
            let tunnel = tunnels.get_mut(&p).unwrap();
            match tunnel.start_next == ' ' {
                true => tunnel.start_next = c,
                false => tunnel.end_next = Some(c),
            }
        }

        prev = Some(c);
    }

    let mut steps = 0;
    let mut next = 0;
    while next < sequence.len() {
        let tunnel = tunnels.get(&sequence[next]).unwrap();
        println!(
            "{} {} -> {}",
            sequence[next],
            sequence[tunnel.exit(next)],
            tunnel.length
        );
        next = tunnel.exit(next) + 1;
        steps += tunnel.length;
    }

    println!("  │  ├─ Part 1: {}", steps);
    //println!("  │  ├─ Part 2: {}",);
    //println!("  │  └─ Part 3: {}",);
}

struct Tunnel {
    start: usize,
    end: usize,
    length: usize,
    start_next: char,
    end_next: Option<char>,
}

impl Tunnel {
    fn new(start: usize) -> Self {
        Self {
            start,
            end: 0,
            length: 0,
            start_next: ' ',
            end_next: None,
        }
    }

    fn close(&mut self, end: usize) {
        self.end = end;
        self.length = end - self.start;
    }

    fn exit(&self, entrance: usize) -> usize {
        match entrance == self.start {
            true => self.end,
            false => self.start,
        }
    }
}
