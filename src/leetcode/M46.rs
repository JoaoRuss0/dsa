struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();

        permutate(&mut permutations, Vec::new(), nums);
        permutations
    }
}

fn permutate(permutations: &mut Vec<Vec<i32>>, current: Vec<i32>, nums: Vec<i32>) {
    if nums.is_empty() {
        permutations.push(current.clone());
        return;
    }

    for n in nums.iter() {
        let mut current = current.clone();
        current.push(*n);

        let mut remaining = nums.clone();
        remaining.remove(nums.iter().position(|x| x == n).unwrap());
        permutate(permutations, current, remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let permutations = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        assert_eq!(permutations, Solution::permute(vec![1, 2, 3]));
    }
}
