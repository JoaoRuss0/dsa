use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            *map.entry(num).or_default() += 1;
        }

        map.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }
}
