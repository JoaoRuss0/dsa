use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let brackets = vec![('(', ')'), ('{', '}'), ('[', ']')]
            .into_iter()
            .collect::<HashMap<char, char>>();

        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if stack.is_empty() {
                stack.push(c);
                continue;
            }

            let popped = stack.pop().unwrap();
            if let Some(&closing) = brackets.get(&popped) {
                if closing == c {
                    continue;
                }
            }
            stack.push(popped);
            stack.push(c);
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_valid("([])".to_string()));
    }
}
