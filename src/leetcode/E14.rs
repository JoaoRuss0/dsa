struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();

        for s in strs.iter().skip(1) {
            if prefix.is_empty() {
                return "".to_string();
            }

            let p_chars = prefix.chars().collect::<Vec<char>>();
            let s_chars = s.chars().collect::<Vec<char>>();

            let mut i = 0;
            while i < p_chars.len() && i < s_chars.len() && p_chars[i] == s_chars[i] {
                i += 1;
            }
            prefix = prefix[0..i].to_string();
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
    }
}
