struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answers = vec![1; nums.len()];

        let mut carried = 1;
        for (i, n) in nums.iter().enumerate() {
            answers[i] *= carried;
            carried *= n;
        }

        carried = 1;
        for (i, n) in nums.iter().enumerate().rev() {
            answers[i] *= carried;
            carried *= n;
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
    }
}
