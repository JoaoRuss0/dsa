pub fn run() {
    println!("  ├─ Day 1 - Inverse Captcha");

    let path = "input/advent_of_code/Y2017/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let nums = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("  │  ├─ Part 1: {}", sum(nums));
    //println!("  │  └─ Part 2: {}", );
}

fn sum(nums: Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..nums.len() {
        if nums[i] == nums[(i + 1) % nums.len()] {
            sum += nums[i];
        }
    }
    sum
}
