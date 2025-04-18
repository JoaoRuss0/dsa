struct Solution;

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut c1 = l1;
        let mut c2 = l2;

        let mut l3: Option<Box<ListNode>> = None;
        let mut c3 = &mut l3;

        let mut carry = 0;
        while c1.is_some() && c2.is_some() {
            if let (Some(s1), Some(s2)) = (&c1, &c2) {
                let mut value = carry + s1.val + s2.val;
                carry = value / 10;
                value %= 10;

                let new = Some(Box::new(ListNode::new(value)));
                c1 = s1.next.clone();
                c2 = s2.next.clone();

                *c3 = new;
                c3 = &mut (*c3).as_mut().unwrap().next;
            }
        }

        let mut leftover = if c1.is_some() { c1 } else { c2 };

        while let Some(l) = &leftover {
            let mut value = carry + l.val;
            carry = value / 10;
            value %= 10;

            let new = Some(Box::new(ListNode::new(value)));
            leftover = l.next.clone();
            *c3 = new;
            c3 = &mut (*c3).as_mut().unwrap().next;
        }

        if carry > 0 {
            let new = Some(Box::new(ListNode::new(carry)));
            *c3 = new;
        }
        l3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut list1 = ListNode::new(1);
        let mut current = &mut list1;

        for val in [2, 3, 4, 5] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list1 = Some(Box::new(list1));

        let mut list2 = ListNode::new(4);
        let mut current = &mut list2;

        for val in [5, 9, 8, 4] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list2 = Some(Box::new(list2));

        let mut list3 = ListNode::new(5);
        let mut current = &mut list3;

        for val in [7, 2, 3, 0, 1] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list3 = Some(Box::new(list3));

        assert_eq!(list3, Solution::add_two_numbers(list1, list2));
    }

    #[test]
    fn test2() {
        let mut list1 = ListNode::new(1);
        let mut current = &mut list1;

        for val in [2, 3] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list1 = Some(Box::new(list1));

        let mut list2 = ListNode::new(4);
        let mut current = &mut list2;

        for val in [5] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list2 = Some(Box::new(list2));

        let mut list3 = ListNode::new(5);
        let mut current = &mut list3;

        for val in [7, 3] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list3 = Some(Box::new(list3));

        assert_eq!(list3, Solution::add_two_numbers(list1, list2));
    }

    #[test]
    fn test3() {
        let mut list1 = ListNode::new(1);
        let mut current = &mut list1;

        for val in [9, 9, 9, 9, 9, 9, 9, 9, 9] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list1 = Some(Box::new(list1));

        let list2 = Some(Box::new(ListNode::new(9)));

        let mut list3 = ListNode::new(0);
        let mut current = &mut list3;

        for val in [0, 0, 0, 0, 0, 0, 0, 0, 0, 1] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        let list3 = Some(Box::new(list3));

        assert_eq!(list3, Solution::add_two_numbers(list1, list2));
    }
}
