struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut triplets: Vec<Vec<i32>> = Vec::new();
        for n in 0..sorted.len() - 2 {
            if n != 0 && sorted[n] == sorted[n - 1] {
                continue;
            }

            let goal = -sorted[n];

            let mut i = n + 1;
            let mut j = nums.len() - 1;

            while i < j {
                let result = sorted[i] + sorted[j];

                match result.cmp(&goal) {
                    std::cmp::Ordering::Less => i += 1,
                    std::cmp::Ordering::Equal => {
                        triplets.push(vec![sorted[n], sorted[i], sorted[j]]);
                        j -= 1;
                        while sorted[j] == sorted[j + 1] && i < j {
                            j -= 1;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        j -= 1;
                        while sorted[j] == sorted[j + 1] && i < j {
                            j -= 1;
                        }
                    }
                }
            }
        }

        triplets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![vec![-2, 0, 2], vec![-2, 1, 1]],
            Solution::three_sum(vec![-2, 0, 1, 1, 2])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            vec![
                vec![-10, 5, 5],
                vec![-5, 0, 5],
                vec![-4, 2, 2],
                vec![-3, -2, 5],
                vec![-3, 1, 2],
                vec![-2, 0, 2]
            ],
            Solution::three_sum(vec![
                2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10
            ])
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }
}
