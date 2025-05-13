use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut layer = HashMap::new();
        layer.insert((0, 0), 1);

        let directions = vec![(0, 1), (1, 0)];
        let mut unique = 0;

        while !layer.is_empty() {
            let mut next_layer = HashMap::new();
            for (&(x, y), count) in layer.iter() {
                if (x, y) == (m - 1, n - 1) {
                    unique += count;
                    continue;
                }

                for (dx, dy) in &directions {
                    let next = (x + dx, y + dy);
                    if !is_position_within_grid(next, (m, n)) {
                        continue;
                    }

                    *next_layer.entry(next).or_insert(0) += count;
                }
            }
            layer = next_layer;
        }

        unique
    }
}

fn is_position_within_grid((x, y): (i32, i32), (mx, my): (i32, i32)) -> bool {
    x >= 0 && y >= 0 && x < mx && y < my
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
