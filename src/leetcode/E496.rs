use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let index_of: HashMap<i32, usize> =
            nums1.iter().enumerate().map(|(i, &j)| (j, i)).collect();

        let mut stack = Vec::new();
        let mut greater: Vec<i32> = vec![-1; nums1.len()];

        nums2.iter().for_each(|&j| {
            while !stack.is_empty() && stack.last().is_some() && stack.last().unwrap() < &j {
                greater[index_of[&stack.pop().unwrap()]] = j;
            }

            if index_of.contains_key(&j) {
                stack.push(j);
            }
        });

        greater
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![-1, 3, -1],
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![5, 2, 6, 9, 11, 3, 7],
            Solution::next_greater_element(
                vec![3, 1, 5, 7, 9, 2, 6],
                vec![1, 2, 3, 5, 6, 7, 9, 11]
            )
        );
    }
}
