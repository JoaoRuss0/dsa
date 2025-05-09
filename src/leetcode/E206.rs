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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut next = head;

        while let Some(mut current) = next.take() {
            next = current.next;
            current.next = prev;
            prev = Some(current);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let next4 = Box::new(ListNode { val: 5, next: None });
        let next3 = Box::new(ListNode {
            val: 4,
            next: Some(next4),
        });
        let next2 = Box::new(ListNode {
            val: 3,
            next: Some(next3),
        });
        let next = Box::new(ListNode {
            val: 2,
            next: Some(next2),
        });
        let head = Box::new(ListNode {
            val: 1,
            next: Some(next),
        });

        let new_head = Solution::reverse_list(Some(head));

        assert_eq!(5, new_head.as_ref().unwrap().val);
        assert_eq!(4, new_head.as_ref().unwrap().next.as_ref().unwrap().val);
        assert_eq!(
            3,
            new_head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val
        );
        assert_eq!(
            2,
            new_head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val
        );
        assert_eq!(
            1,
            new_head
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .val
        );
    }
}
