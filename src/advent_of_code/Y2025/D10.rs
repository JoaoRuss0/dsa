pub fn run() {
    println!("  ├─ Day 10 - Factory");

    let path = "input/advent_of_code/Y2025/D10.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let machines = input.lines().map(Machine::from).collect::<Vec<Machine>>();

    println!(
        "  │  ├─ Part 1: {}",
        machines.iter().map(|m| m.configure() as u32).sum::<u32>()
    );
    //println!("  │  └─ Part 2: {}", );
}

struct Machine {
    light_diagram: LightDiagram,
    button_wiring: Vec<Vec<u16>>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn from(input: &str) -> Self {
        let light_split = input.split("] (").collect::<Vec<&str>>();
        let wiring_joltage_split = light_split[1].split(") {").collect::<Vec<&str>>();
        let wiring_s = wiring_joltage_split[0].split(") (").collect::<Vec<&str>>();
        let joltage_s = wiring_joltage_split[1];

        let mut light_diagram = LightDiagram::from(&light_split[0][1..]);

        let mut button_wiring = Vec::new();
        for w in wiring_s {
            let mut wiring = Vec::new();
            for n in w.split(",") {
                wiring.push(n.parse::<u16>().unwrap());
            }
            button_wiring.push(wiring);
        }

        let mut joltage_requirements = Vec::new();
        for w in joltage_s[..joltage_s.len() - 1].split(",") {
            joltage_requirements.push(w.parse::<u16>().unwrap());
        }

        Self {
            light_diagram,
            button_wiring,
            joltage_requirements,
        }
    }

    fn configure(&self) -> u8 {
        let mut lights = LightDiagram::new(self.light_diagram.state.len());
        let mut used = vec![false; self.button_wiring.len()];
        let mut best = u8::MAX;

        self.tap_buttons(&mut lights, &mut used, 0, &mut best);

        best
    }

    fn tap_buttons(&self, current: &mut LightDiagram, used: &mut [bool], taps: u8, best: &mut u8) {
        if taps >= *best {
            return;
        }

        if *current == self.light_diagram {
            *best = taps;
            return;
        }

        for (i, button) in self.button_wiring.iter().enumerate() {
            if used[i] {
                continue;
            }

            current.apply(&button);
            used[i] = true;

            self.tap_buttons(current, used, taps + 1, best);

            current.apply(&button);
            used[i] = false;
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct LightDiagram {
    state: Vec<bool>,
}

impl LightDiagram {
    fn new(length: usize) -> Self {
        Self {
            state: vec![false; length],
        }
    }

    fn from(string: &str) -> Self {
        let mut state = Vec::new();
        for c in string.chars() {
            match c {
                '.' => state.push(false),
                '#' => state.push(true),
                _ => panic!("Invalid character in light diagram: {}", c),
            }
        }

        Self { state }
    }

    fn apply(&mut self, wiring: &Vec<u16>) {
        for &i in wiring {
            self.state[i as usize] = !self.state[i as usize];
        }
    }
}
