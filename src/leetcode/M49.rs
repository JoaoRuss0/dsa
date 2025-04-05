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
            groups.entry(map).or_insert(Vec::new()).push(s);
        }

        groups.values().cloned().collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ])
        );
    }
}
