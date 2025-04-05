use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for x in nums {
            if map.contains_key(&x) {
                return true;
            }
            map.insert(x, map.len());
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }
}
