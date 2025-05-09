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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        are_nodes_equal(&p, &q)
    }
}

fn are_nodes_equal(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p_inner), Some(q_inner)) => {
            let p_val = p_inner.borrow();
            let q_val = q_inner.borrow();

            p_val.val == q_val.val
                && are_nodes_equal(&p_val.left, &q_val.left)
                && are_nodes_equal(&p_val.right, &q_val.right)
        }
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root1 = TreeNode::new(1);
        let left1 = TreeNode::new(2);
        let right1 = TreeNode::new(3);

        root1.left = Some(Rc::new(RefCell::new(left1)));
        root1.right = Some(Rc::new(RefCell::new(right1)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(2);
        let right2 = TreeNode::new(3);

        root2.left = Some(Rc::new(RefCell::new(left2)));
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert!(Solution::is_same_tree(
            Some(Rc::new(RefCell::new(root1))),
            Some(Rc::new(RefCell::new(root2)))
        ));
    }

    #[test]
    fn test2() {
        let mut root1 = TreeNode::new(1);
        let left1 = TreeNode::new(3);
        let right1 = TreeNode::new(2);

        root1.left = Some(Rc::new(RefCell::new(left1)));
        root1.right = Some(Rc::new(RefCell::new(right1)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(2);
        let right2 = TreeNode::new(3);

        root2.left = Some(Rc::new(RefCell::new(left2)));
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert!(!Solution::is_same_tree(
            Some(Rc::new(RefCell::new(root1))),
            Some(Rc::new(RefCell::new(root2)))
        ));
    }

    #[test]
    fn test3() {
        let mut root1 = TreeNode::new(1);
        let left1 = TreeNode::new(2);

        root1.left = Some(Rc::new(RefCell::new(left1)));

        let mut root2 = TreeNode::new(1);
        let left2 = TreeNode::new(2);
        let right2 = TreeNode::new(3);

        root2.left = Some(Rc::new(RefCell::new(left2)));
        root2.right = Some(Rc::new(RefCell::new(right2)));

        assert!(!Solution::is_same_tree(
            Some(Rc::new(RefCell::new(root1))),
            Some(Rc::new(RefCell::new(root2)))
        ));
    }
}
