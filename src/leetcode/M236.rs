use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        if root == p || root == q {
            return root;
        }

        let node = root?;
        let borrowed = node.borrow();
        let left = Self::lowest_common_ancestor(borrowed.left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(borrowed.right.clone(), p.clone(), q.clone());

        let val = borrowed.val;
        match (left, right) {
            (Some(_), Some(_)) => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(3);

        let mut left = TreeNode::new(5);
        let mut right = TreeNode::new(1);

        let ll = TreeNode::new(6);
        let mut lr = TreeNode::new(2);

        let rl = TreeNode::new(0);
        let rr = TreeNode::new(8);

        let lrl = TreeNode::new(7);
        let lrr = TreeNode::new(4);

        lr.left = Some(Rc::new(RefCell::new(lrl)));
        lr.right = Some(Rc::new(RefCell::new(lrr.clone())));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left.clone())));
        root.right = Some(Rc::new(RefCell::new(right.clone())));

        let root_node = Some(Rc::new(RefCell::new(root)));

        let p = Some(Rc::new(RefCell::new(left)));
        let mut q = Some(Rc::new(RefCell::new(right)));
        assert_eq!(
            3,
            Solution::lowest_common_ancestor(root_node.clone(), p.clone(), q)
                .unwrap()
                .borrow()
                .val
        );

        q = Some(Rc::new(RefCell::new(lrr)));
        assert_eq!(
            5,
            Solution::lowest_common_ancestor(root_node, p, q)
                .unwrap()
                .borrow()
                .val
        );
    }
}
