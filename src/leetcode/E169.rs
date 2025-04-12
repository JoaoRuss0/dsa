struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for num in nums {
            if num == candidate {
                count += 1;
                continue;
            }

            if count == 0 {
                candidate = num;
                count = 1;
                continue;
            }

            count -= 1;
        }

        candidate
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
