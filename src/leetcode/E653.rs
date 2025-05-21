use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        in_order_traversal(root, &mut HashSet::new(), k)
    }
}

fn in_order_traversal(
    node: Option<Rc<RefCell<TreeNode>>>,
    to_search: &mut HashSet<i32>,
    value: i32,
) -> bool {
    if let Some(node) = node {
        let borrowed = node.borrow();
        if to_search.contains(&borrowed.val) {
            return true;
        }

        to_search.insert(value - borrowed.val);
        return in_order_traversal(borrowed.left.clone(), to_search, value)
            || in_order_traversal(borrowed.right.clone(), to_search, value);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(5);

        let mut left = TreeNode::new(3);
        let mut right = TreeNode::new(6);

        let ll = TreeNode::new(2);
        let lr = TreeNode::new(4);

        let rr = TreeNode::new(7);

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert!(Solution::find_target(Some(Rc::new(RefCell::new(root))), 7));
    }

    #[test]
    fn test2() {
        let mut root = TreeNode::new(5);

        let mut left = TreeNode::new(3);
        let mut right = TreeNode::new(6);

        let ll = TreeNode::new(2);
        let lr = TreeNode::new(4);

        let rr = TreeNode::new(7);

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert!(!Solution::find_target(
            Some(Rc::new(RefCell::new(root))),
            14
        ));
    }

    #[test]
    fn test3() {
        let mut root = TreeNode::new(0);

        let mut left = TreeNode::new(-1);
        let mut right = TreeNode::new(2);

        let ll = TreeNode::new(-3);
        let rr = TreeNode::new(4);

        left.left = Some(Rc::new(RefCell::new(ll)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert!(Solution::find_target(Some(Rc::new(RefCell::new(root))), -4));
    }

    #[test]
    fn test4() {
        let mut root = TreeNode::new(2);

        let mut left = TreeNode::new(0);
        let right = TreeNode::new(3);

        let ll = TreeNode::new(-4);
        let lr = TreeNode::new(1);

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert!(Solution::find_target(Some(Rc::new(RefCell::new(root))), -1));
    }
}
