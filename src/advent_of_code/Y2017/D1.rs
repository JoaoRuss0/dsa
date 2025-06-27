pub fn run() {
    println!("  ├─ Day 1 - Inverse Captcha");

    let path = "input/advent_of_code/Y2017/D1.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let nums = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let get_next_index = |cur_idx: usize, nums: &Vec<u32>| -> usize { (cur_idx + 1) % nums.len() };

    println!("  │  ├─ Part 1: {}", sum(&nums, get_next_index));

    let get_next_halfway_around_index =
        |cur_idx: usize, nums: &Vec<u32>| -> usize { (cur_idx + (nums.len() / 2)) % nums.len() };

    println!(
        "  │  └─ Part 2: {}",
        sum(&nums, get_next_halfway_around_index)
    );
}

fn sum(nums: &Vec<u32>, get_digit: fn(usize, &Vec<u32>) -> usize) -> u32 {
    let mut sum = 0;
    for i in 0..nums.len() {
        if nums[i] == nums[get_digit(i, nums)] {
            sum += nums[i];
        }
    }
    sum
}
