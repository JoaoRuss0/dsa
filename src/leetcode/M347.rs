use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency: HashMap<i32, i32> = HashMap::new();

        nums.iter()
            .for_each(|&n| *frequency.entry(n).or_default() += 1);

        let mut sorted: Vec<_> = frequency.into_iter().collect();
        sorted.sort_by(|e1, e2| e2.1.cmp(&e1.1));
        sorted
            .into_iter()
            .take(k as usize)
            .map(|(n, _)| n)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![1, 2],
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
        );
    }

    #[test]
    fn test2() {
        assert_eq!(vec![-1], Solution::top_k_frequent(vec![-1, -1], 1));
    }
}
