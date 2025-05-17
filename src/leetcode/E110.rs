use std::cell::RefCell;
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_subtree_balanced(root).1
    }
}

fn is_subtree_balanced(node: Option<Rc<RefCell<TreeNode>>>) -> (usize, bool) {
    if node.is_none() {
        return (0, true);
    }

    let inner = node.unwrap();
    let borrowed = inner.borrow();
    let left = is_subtree_balanced(borrowed.left.clone());
    if !left.1 {
        return (0, false);
    }

    let right = is_subtree_balanced(borrowed.right.clone());
    if !right.1 {
        return (0, false);
    }

    (left.0.max(right.0) + 1, right.0.abs_diff(left.0) <= 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(3);

        let left = TreeNode::new(9);
        let mut right = TreeNode::new(20);

        let rl = TreeNode::new(15);
        let rr = TreeNode::new(7);

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_balanced(None));
    }

    #[test]
    fn test3() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let right = TreeNode::new(2);

        let mut ll = TreeNode::new(3);
        let lr = TreeNode::new(3);

        let lll = TreeNode::new(4);
        let llr = TreeNode::new(4);

        ll.left = Some(Rc::new(RefCell::new(lll)));
        ll.right = Some(Rc::new(RefCell::new(llr)));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert!(!Solution::is_balanced(Some(Rc::new(RefCell::new(root)))));
    }
}
