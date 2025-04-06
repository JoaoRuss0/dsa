struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut sorted = nums.clone();
        sorted.sort();

        let mut longest: i32 = 1;
        let mut cur_longest: i32 = 1;
        for (i, n) in sorted.iter().enumerate().skip(1) {
            if *n == sorted[i - 1] {
                continue;
            }

            if *n == sorted[i - 1] + 1 {
                cur_longest += 1;
                continue;
            }

            if cur_longest > longest {
                longest = cur_longest;
                cur_longest = 1;
                continue;
            }

            cur_longest = 1;
        }

        if cur_longest > longest {
            longest = cur_longest;
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(3, Solution::longest_consecutive(vec![1, 0, 1, 2]));
    }

    #[test]
    fn test3() {
        assert_eq!(
            4,
            Solution::longest_consecutive(vec![9, 1, -3, 2, 4, 8, 3, -1, 6, -2, -4, 7])
        );
    }
}
