use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: i32,
    nums: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
        nums.into_iter().for_each(|num| {
            heap.push(Reverse(num));
        });

        while heap.len() > k as usize {
            heap.pop();
        }

        Self { k, nums: heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));

        while self.nums.len() > self.k as usize {
            self.nums.pop();
        }

        self.nums.peek().unwrap().0
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
