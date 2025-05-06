struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut sorted = false;
        while !sorted {
            let mut switched = false;

            let mut i = 0;
            while i < nums.len() - 1 {
                let current = nums[i];
                let next = nums[i + 1];

                if current > next {
                    nums[i] = next;
                    nums[i + 1] = current;
                    switched = true;
                }

                i += 1;
            }

            if !switched {
                sorted = true;
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
}
