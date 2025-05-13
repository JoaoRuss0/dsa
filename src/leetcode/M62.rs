struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut grid = vec![vec![1; n]; m];

        for x in 1..m {
            for y in 1..n {
                grid[x][y] = grid[x - 1][y] + grid[x][y - 1];
            }
        }

        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, Solution::unique_paths(3, 2));
    }

    #[test]
    fn test2() {
        assert_eq!(28, Solution::unique_paths(3, 7));
    }
}
