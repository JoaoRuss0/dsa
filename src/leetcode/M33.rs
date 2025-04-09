struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut index = -1;

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = (l + r) / 2;
            if nums[m] == target {
                index = m as i32;
                break;
            }

            if nums[l] <= nums[m] {
                if nums[m] < target || nums[l] > target {
                    l = m + 1;
                    continue;
                }
                r = m - 1;
                continue;
            }

            if nums[m] > target || nums[r] < target {
                r = m - 1;
                continue;
            }
            l = m + 1;
        }

        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(0, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4));
    }

    #[test]
    fn test2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn test3() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn test4() {
        assert_eq!(-1, Solution::search(vec![1], 0));
    }

    #[test]
    fn test5() {
        assert_eq!(-1, Solution::search(vec![0], 2));
    }
}
