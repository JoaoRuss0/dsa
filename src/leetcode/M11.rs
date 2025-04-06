struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut maximum = 0;
        while l < r {
            let current = height[l].min(height[r]) * (r - l) as i32;
            maximum = maximum.max(current);

            if height[l] > height[r] {
                r -= 1;
                continue;
            }
            l += 1;
        }
        maximum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn test2() {
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }
}
