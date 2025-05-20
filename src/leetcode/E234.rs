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

struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while fast.as_ref().unwrap().next.is_some()
            && fast.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
        {
            slow = slow.unwrap().next.clone();
            fast = fast.unwrap().next.unwrap().next.clone();
        }

        let mut middle = slow.unwrap().next.take();

        let mut end = None;
        let mut previous = None;
        while let Some(mut curr) = middle {
            let next = curr.next.take();
            curr.next = previous;

            previous = Some(curr);
            middle = next;

            if middle.is_none() {
                end = previous;
                break;
            }
        }

        let mut start = head.clone();
        while let (Some(e), Some(s)) = (end, start) {
            if e.val != s.val {
                return false;
            }
            start = s.next;
            end = e.next;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut head = ListNode::new(1);
        let mut second = ListNode::new(2);
        let mut third = ListNode::new(1);
        let fourth = ListNode::new(1);

        third.next = Some(Box::new(fourth));
        second.next = Some(Box::new(third));
        head.next = Some(Box::new(second));

        assert!(!Solution::is_palindrome(Some(Box::new(head))));
    }

    #[test]
    fn test2() {
        let mut head = ListNode::new(1);
        let mut second = ListNode::new(2);
        let mut third = ListNode::new(2);
        let fourth = ListNode::new(1);

        third.next = Some(Box::new(fourth));
        second.next = Some(Box::new(third));
        head.next = Some(Box::new(second));

        assert!(Solution::is_palindrome(Some(Box::new(head))));
    }

    #[test]
    fn test3() {
        let mut head = ListNode::new(1);
        let mut second = ListNode::new(3);
        let mut third = ListNode::new(4);
        let mut fourth = ListNode::new(4);
        let fifth = ListNode::new(1);

        fourth.next = Some(Box::new(fifth));
        third.next = Some(Box::new(fourth));
        second.next = Some(Box::new(third));
        head.next = Some(Box::new(second));

        assert!(!Solution::is_palindrome(Some(Box::new(head))));
    }

    #[test]
    fn test4() {
        let mut head = ListNode::new(1);
        let mut second = ListNode::new(0);
        let third = ListNode::new(1);

        second.next = Some(Box::new(third));
        head.next = Some(Box::new(second));

        assert!(Solution::is_palindrome(Some(Box::new(head))));
    }

    #[test]
    fn test5() {
        let mut head = ListNode::new(1);
        let second = ListNode::new(1);

        head.next = Some(Box::new(second));

        assert!(Solution::is_palindrome(Some(Box::new(head))));
    }
}
