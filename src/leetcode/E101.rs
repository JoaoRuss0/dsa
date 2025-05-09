use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.unwrap();
        let borrowed = root.borrow();
        let mut deque = VecDeque::new();
        deque.push_back(borrowed.left.clone());
        deque.push_back(borrowed.right.clone());

        loop {
            if deque.is_empty() {
                break;
            }

            let mut next = VecDeque::new();

            while let (Some(l), Some(r)) = (deque.pop_front(), deque.pop_back()) {
                match (l, r) {
                    (None, None) => continue,
                    (Some(l), Some(r)) => {
                        let l_borrowed = l.borrow();
                        let r_borrowed = r.borrow();

                        if l_borrowed.val != r_borrowed.val {
                            return false;
                        }

                        next.push_front(l_borrowed.left.clone());
                        next.push_front(l_borrowed.right.clone());
                        next.push_back(r_borrowed.right.clone());
                        next.push_back(r_borrowed.left.clone());
                    }
                    _ => return false,
                }
            }
            deque = next;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::E101::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(2);

        let ll = TreeNode::new(3);
        let lr = TreeNode::new(4);

        let rl = TreeNode::new(4);
        let rr = TreeNode::new(3);

        right.right = Some(Rc::new(RefCell::new(rr)));
        right.left = Some(Rc::new(RefCell::new(rl)));

        left.right = Some(Rc::new(RefCell::new(lr)));
        left.left = Some(Rc::new(RefCell::new(ll)));

        root.right = Some(Rc::new(RefCell::new(right)));
        root.left = Some(Rc::new(RefCell::new(left)));

        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn test2() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(2);

        let lr = TreeNode::new(3);
        let rr = TreeNode::new(3);

        right.right = Some(Rc::new(RefCell::new(rr)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.right = Some(Rc::new(RefCell::new(right)));
        root.left = Some(Rc::new(RefCell::new(left)));

        assert!(!Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn test3() {
        let mut root = TreeNode::new(2);

        let mut left = TreeNode::new(3);
        let mut right = TreeNode::new(3);

        let ll = TreeNode::new(4);
        let mut lr = TreeNode::new(5);

        let mut rl = TreeNode::new(5);
        let rr = TreeNode::new(4);

        let lrl = TreeNode::new(8);
        let lrr = TreeNode::new(9);

        let rll = TreeNode::new(9);
        let rlr = TreeNode::new(8);

        rl.left = Some(Rc::new(RefCell::new(rll)));
        rl.right = Some(Rc::new(RefCell::new(rlr)));

        lr.left = Some(Rc::new(RefCell::new(lrl)));
        lr.right = Some(Rc::new(RefCell::new(lrr)));

        right.right = Some(Rc::new(RefCell::new(rr)));
        right.left = Some(Rc::new(RefCell::new(rl)));

        left.right = Some(Rc::new(RefCell::new(lr)));
        left.left = Some(Rc::new(RefCell::new(ll)));

        root.right = Some(Rc::new(RefCell::new(right)));
        root.left = Some(Rc::new(RefCell::new(left)));

        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))));
    }
}
