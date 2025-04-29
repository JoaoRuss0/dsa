struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(l1), Some(l2)) => {
                let mut head = Box::new(ListNode::new(0));
                let mut current = &mut head;

                let mut left_iterator = Some(l1);
                let mut right_iterator = Some(l2);

                while left_iterator.is_some() && right_iterator.is_some() {
                    let next = if left_iterator.as_ref().unwrap().val
                        <= right_iterator.as_ref().unwrap().val
                    {
                        let mut node = left_iterator.unwrap();
                        left_iterator = node.next.take();
                        node
                    } else {
                        let mut node = right_iterator.unwrap();
                        right_iterator = node.next.take();
                        node
                    };
                    current.next = Some(next);
                    current = current.next.as_mut().unwrap();
                }

                current.next = if left_iterator.is_some() {
                    left_iterator
                } else {
                    right_iterator
                };
                head.next
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut list_one = ListNode::new(1);
        let mut n_one = ListNode::new(2);
        let nn_one = ListNode::new(4);
        n_one.next = Some(Box::new(nn_one));
        list_one.next = Some(Box::new(n_one));

        let mut list_two = ListNode::new(1);
        let mut n_two = ListNode::new(3);
        let nn_two = ListNode::new(4);
        n_two.next = Some(Box::new(nn_two));
        list_two.next = Some(Box::new(n_two));

        let mut list_three = ListNode::new(1);
        let mut n_three = ListNode::new(1);
        let mut nn_three = ListNode::new(2);
        let mut nnn_three = ListNode::new(3);
        let mut nnnn_three = ListNode::new(4);
        let nnnnn_three = ListNode::new(4);

        nnnn_three.next = Some(Box::new(nnnnn_three));
        nnn_three.next = Some(Box::new(nnnn_three));
        nn_three.next = Some(Box::new(nnn_three));
        n_three.next = Some(Box::new(nn_three));
        list_three.next = Some(Box::new(n_three));

        assert_eq!(
            Some(Box::new(list_three)),
            Solution::merge_two_lists(Some(Box::new(list_one)), Some(Box::new(list_two)))
        );
    }
}
