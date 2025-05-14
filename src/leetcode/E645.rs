struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut map = vec![0; nums.len()];
        for n in nums.into_iter() {
            map[n as usize - 1] += 1;
        }

        let mut missing = 0;
        let mut duplicate = 0;
        for (i, n) in map.into_iter().enumerate() {
            if n == 0 {
                missing = i + 1;
                continue;
            }

            if n == 2 {
                duplicate = i + 1;
            }
        }
        vec![duplicate as i32, missing as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2]));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![2, 1], Solution::find_error_nums(vec![3, 2, 2]));
    }
}
