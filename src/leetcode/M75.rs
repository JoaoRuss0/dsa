struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        let mut i = 0;
        while right >= 0 && i <= right as usize {
            match nums[i] {
                0 => {
                    nums.swap(i, left);
                    left += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, right as usize);
                    right -= 1;
                }
                _ => i += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut colors = vec![2, 1, 0, 1, 2, 0];
        Solution::sort_colors(&mut colors);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], colors);
    }

    #[test]
    fn test2() {
        let mut colors = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut colors);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], colors);
    }

    #[test]
    fn test3() {
        let mut colors = vec![0, 2, 0, 1];
        Solution::sort_colors(&mut colors);
        assert_eq!(vec![0, 0, 1, 2], colors);
    }

    #[test]
    fn test4() {
        let mut colors = vec![2];
        Solution::sort_colors(&mut colors);
        assert_eq!(vec![2], colors);
    }

    #[test]
    fn test5() {
        let mut colors = vec![2, 2];
        Solution::sort_colors(&mut colors);
        assert_eq!(vec![2, 2], colors);
    }
}
