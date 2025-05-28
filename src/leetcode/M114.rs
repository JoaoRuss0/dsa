use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut left = node.borrow_mut().left.take();
            let mut right = node.borrow_mut().right.take();

            if left.is_some() {
                Self::flatten(&mut left);
                node.borrow_mut().right = left;
            }

            if right.is_some() {
                Self::flatten(&mut right);

                let mut current = node.clone();
                while current.borrow().right.is_some() {
                    let right_cloned = current.borrow().right.as_ref().unwrap().clone();
                    current = right_cloned;
                }

                current.borrow_mut().right = right;
            }

            node.borrow_mut().left = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(5);

        let ll = TreeNode::new(3);
        let lr = TreeNode::new(4);

        let rr = TreeNode::new(6);

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut to_flatten = Some(Rc::new(RefCell::new(root.clone())));
        Solution::flatten(&mut to_flatten);

        let mut first = TreeNode::new(1);
        let mut second = TreeNode::new(2);
        let mut third = TreeNode::new(3);
        let mut fourth = TreeNode::new(4);
        let mut fifth = TreeNode::new(5);
        let sixth = TreeNode::new(6);

        fifth.right = Some(Rc::new(RefCell::new(sixth)));
        fourth.right = Some(Rc::new(RefCell::new(fifth)));
        third.right = Some(Rc::new(RefCell::new(fourth)));
        second.right = Some(Rc::new(RefCell::new(third)));
        first.right = Some(Rc::new(RefCell::new(second)));

        assert_eq!(Some(Rc::new(RefCell::new(first))), to_flatten);
    }
}
