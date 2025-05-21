use crate::leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while fast.is_some() && fast.clone().unwrap().next.is_some() {
            slow = slow.unwrap().next;
            fast = fast.unwrap().next?.next;
        }
        slow
    }
}
