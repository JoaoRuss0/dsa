use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        find_kth_smallest(root, &mut k)
    }
}

fn find_kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> i32 {
    if let Some(node) = root {
        let borrowed = node.borrow();

        let mut returned = find_kth_smallest(borrowed.left.clone(), k);
        if returned != -1 {
            return returned;
        }

        if *k == 1 {
            return borrowed.val;
        }

        *k -= 1;

        returned = find_kth_smallest(borrowed.right.clone(), k);
        if returned != -1 {
            return returned;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(3);

        let mut left = TreeNode::new(1);
        let right = TreeNode::new(4);

        let lr = TreeNode::new(2);

        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            3,
            Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 3)
        );
    }

    #[test]
    fn test2() {
        let mut root = TreeNode::new(5);

        let mut left = TreeNode::new(3);
        let right = TreeNode::new(6);

        let mut ll = TreeNode::new(2);
        let lr = TreeNode::new(4);

        let lll = TreeNode::new(1);

        ll.left = Some(Rc::new(RefCell::new(lll)));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            3,
            Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 3)
        );
    }

    #[test]
    fn test3() {
        let mut root = TreeNode::new(5);

        let mut left = TreeNode::new(3);
        let right = TreeNode::new(6);

        let mut ll = TreeNode::new(2);
        let lr = TreeNode::new(4);

        let lll = TreeNode::new(1);

        ll.left = Some(Rc::new(RefCell::new(lll)));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut k = 7;

        assert_eq!(
            -1,
            Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 7)
        );
    }
}
