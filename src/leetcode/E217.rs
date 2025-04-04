use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.iter().collect::<HashSet<&i32>>().len() < nums.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }
}
