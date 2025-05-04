use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u64; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut map = [0; 26];
            for c in s.chars() {
                map[c as usize - b'a' as usize] += 1;
            }
            groups.entry(map).or_default().push(s);
        }

        groups.values().cloned().collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let actual = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);

        let expected = [
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
        ];

        assert_eq!(actual.len(), expected.len());
        expected.iter().for_each(|v| assert!(actual.contains(v)));
    }
}
