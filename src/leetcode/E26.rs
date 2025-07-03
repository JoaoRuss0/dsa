struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        let mut i = 1;
        let mut j = 1;
        while j < nums.len() {
            if nums[i - 1] != nums[j] {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }

        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 2, 2, 3, 4, 5];
        assert_eq!(5, Solution::remove_duplicates(&mut nums));
    }
}
