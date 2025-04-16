use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let original_color = image[sr as usize][sc as usize];
        let mut flooded = image.clone();

        let rotations = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((sr as usize, sc as usize));
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            flooded[x][y] = color;
            visited.insert((x, y));

            for &(rx, ry) in &rotations {
                let (nx, ny) = (x as i32 + rx, y as i32 + ry);
                if nx < 0 || ny < 0 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);
                if nx < flooded.len()
                    && ny < flooded[0].len()
                    && !visited.contains(&(nx, ny))
                    && flooded[nx][ny] == original_color
                {
                    queue.push_back((nx, ny));
                }
            }
        }

        flooded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        );
    }
}
