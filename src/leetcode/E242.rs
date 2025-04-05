struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = [0; 26];

        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        while let (Some(c1), Some(c2)) = (s_chars.next(), t_chars.next()) {
            map[(c1 as u8 - b'a') as usize] += 1;
            map[(c2 as u8 - b'a') as usize] -= 1;
        }

        for i in &map {
            if *i != 0 {
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
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}
