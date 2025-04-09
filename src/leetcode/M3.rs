use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut max = 0;
        let mut contained: HashSet<char> = HashSet::new();

        let chars = s.chars().collect::<Vec<char>>();
        while r < s.len() {
            while contained.contains(&chars[r]) {
                contained.remove(&chars[l]);
                l += 1;
            }

            contained.insert(chars[r]);
            max = max.max(r - l + 1);
            r += 1;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }

    #[test]
    fn test3() {
        assert_eq!(2, Solution::length_of_longest_substring("au".to_string()));
    }

    #[test]
    fn test4() {
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
    }
}
