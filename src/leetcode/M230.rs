use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut count = 0;
        let mut current = root;

        while current.is_some() || !stack.is_empty() {
            while current.is_some() {
                stack.push(current.clone());

                let node = current.as_ref().unwrap().clone();
                current = node.borrow().left.clone();
            }

            let popped = stack.pop().unwrap().unwrap();

            count += 1;
            if k == count {
                return popped.borrow().val;
            }

            current = popped.borrow().right.clone();
        }

        -1
    }
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

        assert_eq!(
            -1,
            Solution::kth_smallest(Some(Rc::new(RefCell::new(root))), 7)
        );
    }
}
