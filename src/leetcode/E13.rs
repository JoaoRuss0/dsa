struct Solution;

enum Numeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}
impl Numeral {
    fn of(num: char) -> Numeral {
        match num {
            'I' => Numeral::I,
            'V' => Numeral::V,
            'X' => Numeral::X,
            'L' => Numeral::L,
            'C' => Numeral::C,
            'D' => Numeral::D,
            'M' => Numeral::M,
            _ => panic!("Invalid numeral"),
        }
    }

    fn value(&self) -> i32 {
        match self {
            Numeral::I => 1,
            Numeral::V => 5,
            Numeral::X => 10,
            Numeral::L => 50,
            Numeral::C => 100,
            Numeral::D => 500,
            Numeral::M => 1000,
        }
    }

    fn is_lower(&self, n: Numeral) -> bool {
        self.value() < n.value()
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0;
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..s.len() {
            let curr = Numeral::of(*chars.get(i).unwrap());
            if i == s.len() - 1 {
                num += curr.value();
                continue;
            }

            let next = Numeral::of(*chars.get(i + 1).unwrap());
            num += if curr.is_lower(next) {
                -curr.value()
            } else {
                curr.value()
            };
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn test3() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
