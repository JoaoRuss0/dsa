struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut count = 0;
        let mut number = n;

        while number != 0 {
            count += number % 2;
            number /= 2;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::hamming_weight(128));
    }

    #[test]
    fn test2() {
        assert_eq!(4, Solution::hamming_weight(45));
    }
}
