pub fn run() {
    println!("  ├─ Problem 14 - Crucial Crafting");

    let path = "input/codyssi/journey_to_atlantis/P14.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let items: Vec<Item> = input.lines().map(Item::from).collect();

    let mut sorted = items.clone();
    sorted.sort_by(|prev, next| {
        next.quality
            .cmp(&prev.quality)
            .then(next.cost.cmp(&prev.cost))
    });
    println!(
        "  │  ├─ Part 1: {}",
        sorted
            .iter()
            .take(5)
            .map(|i| i.unique_materials)
            .sum::<u32>()
    );

    //println!("  │  ├─ Part 2: {}", );
    //println!("  │  └─ Part 3: {}", );
}

#[derive(Clone, Debug)]
struct Item {
    id: String,
    quality: u32,
    cost: u32,
    unique_materials: u32,
}

impl Item {
    fn from(line: &str) -> Self {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let id = parts[0].split(" ").collect::<Vec<&str>>()[1].trim();

        let properties = parts[1].split(", ").collect::<Vec<&str>>();
        let quality = properties[0].split(" : ").collect::<Vec<&str>>()[1];
        let cost = properties[1].split(" : ").collect::<Vec<&str>>()[1];
        let unique_materials = properties[2].split(" : ").collect::<Vec<&str>>()[1];

        Self {
            id: id.to_string(),
            quality: quality.parse::<u32>().unwrap(),
            cost: cost.parse::<u32>().unwrap(),
            unique_materials: unique_materials.parse::<u32>().unwrap(),
        }
    }
}
