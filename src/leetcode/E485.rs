struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut current = 0;
        for n in nums {
            if n == 1 {
                current += 1;
                continue;
            }

            max = max.max(current);
            current = 0;
        }

        max = max.max(current);
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
        );
    }
}
