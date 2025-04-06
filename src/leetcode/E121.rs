struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut lowest = prices[0];

        for p in prices.into_iter().skip(1) {
            let current = p - lowest;
            if current < 0 {
                lowest = p;
            } else if current > profit {
                profit = current;
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }
}
