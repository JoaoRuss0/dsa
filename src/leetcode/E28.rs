struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }

        let c_haystack = haystack.chars().collect::<Vec<char>>();
        let c_needle = needle.chars().collect::<Vec<char>>();

        for i in 0..haystack.len() - needle.len() + 1 {
            let mut j = 0;
            let mut k = i;

            if c_haystack[k] != c_needle[j] {
                continue;
            }

            while j < needle.len() && k < haystack.len() && c_haystack[k] == c_needle[j] {
                j += 1;
                k += 1;
            }

            if j != needle.len() {
                continue;
            }
            return i as i32;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            0,
            Solution::str_str("sadbutsad".to_string(), "sad".to_string())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(0, Solution::str_str("a".to_string(), "a".to_string()));
    }
}
