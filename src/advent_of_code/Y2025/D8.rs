use std::collections::{BTreeMap, HashMap};

pub fn run() {
    println!("  ├─ Day 8 - Playground");

    let path = "input/advent_of_code/Y2025/D8.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let boxes = input
        .lines()
        .map(JunctionBox::from)
        .collect::<Vec<JunctionBox>>();

    let mut distances = BTreeMap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let distance = boxes[i].distance(&boxes[j]) as usize;
            distances
                .entry(distance)
                .or_insert(Vec::new())
                .push((&boxes[i], &boxes[j]));
        }
    }

    let mut boxes_to_circuit: HashMap<&JunctionBox, usize> = HashMap::new();
    let mut circuits: HashMap<usize, Vec<&JunctionBox>> = HashMap::new();

    let mut connections = 0;
    let mut circuit_product = 0;

    let mut last_two = 0;

    let calculate_circuit_len_product = |circuits: &HashMap<usize, Vec<&JunctionBox>>| -> usize {
        let (mut a, mut b, mut c) = (0, 0, 0);
        for circuit in circuits.values() {
            let len = circuit.len();
            if a < len {
                c = b;
                b = a;
                a = len;
            } else if b < len {
                c = b;
                b = len;
            } else if c < len {
                c = len;
            }
        }
        a * b * c
    };

    // ANOTHER STACK BASED APPROACH USING VECTORS:

    // let mut biggest_circuits = Vec::new();
    // let mut helper = Vec::new();
    //
    // for circuit in circuits.values() {
    //     while let Some(len) = biggest_circuits.pop() {
    //         match len < circuit.len() {
    //             true => helper.push(len),
    //             false => {
    //                 biggest_circuits.push(len);
    //                 break;
    //             }
    //         }
    //     }
    //
    //     if biggest_circuits.len() < 3 {
    //         biggest_circuits.push(circuit.len());
    //     }
    //
    //     while biggest_circuits.len() < 3 {
    //         match helper.pop() {
    //             Some(len) => biggest_circuits.push(len),
    //             None => break,
    //         }
    //     }
    // }

    let mut index = 0;
    'outer: for boxes_for_distance in distances.values() {
        for &(b1, b2) in boxes_for_distance {
            let b1_index = boxes_to_circuit.get(b1).copied();
            let b2_index = boxes_to_circuit.get(b2).copied();

            connections += 1;

            match (b1_index, b2_index) {
                (Some(b1_index), Some(b2_index)) => {
                    if b1_index != b2_index {
                        let mut boxes_circuit_2 = circuits.remove(&b2_index).unwrap();
                        while let Some(box_circuit_2) = boxes_circuit_2.pop() {
                            circuits.get_mut(&b1_index).unwrap().push(box_circuit_2);
                            *boxes_to_circuit.get_mut(box_circuit_2).unwrap() = b1_index;
                        }
                    }
                }
                (Some(circuit_index), None) => {
                    boxes_to_circuit.insert(b2, circuit_index);
                    circuits.get_mut(&circuit_index).unwrap().push(b2);
                }
                (None, Some(circuit_index)) => {
                    boxes_to_circuit.insert(b1, circuit_index);
                    circuits.get_mut(&circuit_index).unwrap().push(b1);
                }
                (None, None) => {
                    let mut circuit_boxes = Vec::new();
                    circuit_boxes.push(b1);
                    circuit_boxes.push(b2);

                    circuits.insert(index, circuit_boxes);

                    boxes_to_circuit.insert(b1, index);
                    boxes_to_circuit.insert(b2, index);

                    index += 1;
                }
            }

            if connections == 1000 {
                circuit_product = calculate_circuit_len_product(&circuits);
            }

            if boxes_to_circuit.keys().len() == boxes.len() {
                last_two = b1.x * b2.x;
                break 'outer;
            }
        }
    }

    println!("  │  ├─ Part 1: {}", circuit_product);
    println!("  │  └─ Part 2: {}", last_two);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    fn from(line: &str) -> Self {
        let split = line
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Self {
            x: split[0],
            y: split[1],
            z: split[2],
        }
    }

    fn distance(&self, other: &Self) -> f32 {
        ((self.x as f32 - other.x as f32).powi(2)
            + (self.y as f32 - other.y as f32).powi(2)
            + (self.z as f32 - other.z as f32).powi(2))
        .sqrt()
    }
}
