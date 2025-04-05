struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<_>>();

        let len = chars.len();
        for i in 0..len.div_ceil(2) {
            if chars[i] != chars[len - i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::is_palindrome("abcd".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_palindrome("abba".to_string()));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }
}
