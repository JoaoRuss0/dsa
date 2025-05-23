struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut temp = Self {
            k,
            nums: Vec::new(),
        };

        for num in nums {
            temp.add(num);
        }

        temp
    }

    fn add(&mut self, val: i32) -> i32 {
        let mut temp = Vec::new();
        while let Some(popped) = self.nums.pop() {
            if popped < val {
                temp.push(popped);
            } else {
                self.nums.push(popped);
                break;
            }
        }

        if self.nums.len() < self.k as usize {
            self.nums.push(val);
        }

        while let Some(popped) = temp.pop() {
            if self.nums.len() < self.k as usize {
                self.nums.push(popped);
            } else {
                break;
            }
        }

        self.nums[self.nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::E703::KthLargest;

    #[test]
    fn test1() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, kth_largest.add(3));
        assert_eq!(5, kth_largest.add(5));
        assert_eq!(5, kth_largest.add(10));
        assert_eq!(8, kth_largest.add(9));
        assert_eq!(8, kth_largest.add(4));
    }
}
