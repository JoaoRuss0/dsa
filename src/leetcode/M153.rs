struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            if nums[l] < nums[r] {
                min = min.min(nums[l]);
                break;
            }

            let m = (l + r) / 2;
            min = min.min(nums[m]);

            if nums[l] <= nums[m] {
                l = m + 1;
                continue;
            }
            r = m - 1;
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::find_min(vec![3, 4, 5, 1, 2]));
    }
}
