struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.trim().chars().collect();
        if chars.is_empty() {
            return 0;
        }

        let (sign, default) = Sign::of(chars[0]);

        let mut stack = Vec::new();
        for c in chars.into_iter().skip(if default { 0 } else { 1 }) {
            if !c.is_ascii_digit() {
                break;
            }

            stack.push(c as i32 - '0' as i32);
        }

        let max_but_first_digit = match sign {
            Sign::Positive => 147483648 - 1,
            Sign::Negative => 147483648,
        };

        let mut num: i32 = 0;
        let mut i = 0;
        while let Some(digit) = stack.pop() {
            if digit == 0 {
                i += 1;
                continue;
            }

            if i > 9 || i == 9 && digit > 2 || digit == 2 && num.abs() > max_but_first_digit {
                return match sign {
                    Sign::Positive => i32::MAX,
                    Sign::Negative => i32::MIN,
                };
            }

            let to_sum = digit * i32::pow(10, i as u32);
            match sign {
                Sign::Positive => num += to_sum,
                Sign::Negative => num -= to_sum,
            };
            i += 1;
        }

        num
    }
}

enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn of(sign_candidate: char) -> (Self, bool) {
        match sign_candidate {
            '-' => (Sign::Negative, false),
            '+' => (Sign::Positive, false),
            _ => (Sign::Positive, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(20, Solution::my_atoi("20".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(-420, Solution::my_atoi("-420".to_string()));
    }

    #[test]
    fn test3() {
        assert_eq!(i32::MIN, Solution::my_atoi("-91283472332".to_string()));
    }

    #[test]
    fn test4() {
        assert_eq!(i32::MIN, Solution::my_atoi("-2147483649".to_string()));
    }

    #[test]
    fn test5() {
        assert_eq!(i32::MIN, Solution::my_atoi("-6147483648".to_string()));
    }
}
