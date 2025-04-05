use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];

            if map.contains_key(&complement) {
                return vec![map[&complement] as i32, i as i32];
            }
            map.insert(nums[i], i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 18));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn test3() {
        assert_eq!(vec![2, 4], Solution::two_sum(vec![-1, -2, -3, -4, -5], -8));
    }
}
