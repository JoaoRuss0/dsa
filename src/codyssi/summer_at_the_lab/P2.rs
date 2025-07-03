pub fn run() {
    println!("  ├─ Problem 2 - Sensors and Circuits");

    let path = "input/codyssi/summer_at_the_lab/P2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let sensors = input
        .lines()
        .map(|line| line == "TRUE")
        .collect::<Vec<bool>>();

    let true_sensor_ids_sum = sensors
        .iter()
        .filter(|&s| *s)
        .enumerate()
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("  │  ├─ Part 1: {true_sensor_ids_sum}");

    let get_gate_output = |(i, inputs): (usize, &[bool])| -> bool {
        match i % 2 == 0 {
            true => inputs[0] && inputs[1],
            false => inputs[0] || inputs[1],
        }
    };

    let true_gate_output_count = sensors
        .chunks(2)
        .enumerate()
        .map(get_gate_output)
        .filter(|&x| x)
        .count();

    println!("  │  ├─ Part 2: {true_gate_output_count}");

    let mut true_gate_output_layer_count = 0;
    let mut layer = sensors.clone();

    while layer.len() != 1 {
        let next_layer = layer
            .chunks(2)
            .enumerate()
            .map(get_gate_output)
            .collect::<Vec<bool>>();

        true_gate_output_layer_count += next_layer.iter().filter(|&i| *i).count();

        layer = next_layer;
    }

    println!(
        "  │  └─ Part 3: {}",
        true_gate_output_layer_count + input.lines().filter(|line| *line == "TRUE").count()
    );
}
