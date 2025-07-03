struct Solution;

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let mut i = m;
        let mut j = n;

        while j > 0 {
            if i > 0 && nums1[i as usize - 1] > nums2[j as usize - 1] {
                nums1[(i + j - 1) as usize] = nums1[i as usize - 1];
                i -= 1;
                continue;
            }

            nums1[(i + j - 1) as usize] = nums2[j as usize - 1];
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        let expected = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn test2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        Solution::merge(&mut nums1, 1, &mut nums2, 0);

        let expected = vec![1];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn test3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];

        Solution::merge(&mut nums1, 0, &mut nums2, 1);

        let expected = vec![1];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn test4() {
        let mut nums1 = vec![2, 0];
        let mut nums2 = vec![1];

        Solution::merge(&mut nums1, 1, &mut nums2, 1);

        let expected = vec![1, 2];
        assert_eq!(expected, nums1);
    }
}
