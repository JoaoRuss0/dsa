struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut greater: Vec<i32> = Vec::new();

        for i in nums1 {
            let mut index_of = nums2.len();
            for (index, &j) in nums2.iter().enumerate() {
                if j == i {
                    index_of = index;
                    break;
                }
            }

            if index_of == nums2.len() {
                continue;
            }

            let mut found = false;

            for &k in nums2[index_of + 1..].iter() {
                if k > i {
                    greater.push(k);
                    found = true;
                    break;
                }
            }

            if !found {
                greater.push(-1);
            }
        }

        greater
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![-1, 3, -1],
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
    }
}
